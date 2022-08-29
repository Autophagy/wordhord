I've somehow managed to convince several companies over the last few years
to hire me as a devops/infra engineer, which mostly involves using incantations
to transform configuration files into invoices. One of those incantations is
[Terraform](https://www.terraform.io/), an infrastructure-as-code tool that
integrates with lots of cloud services.

Working with Terraform always requires the involvement of some credentials for
whatever cloud infrastructure service you're targeting, like the access and
secret access keys for AWS. There's a few ways of getting these credentials into
terraform. One way is by by adding them into the provider using sensitive variables
and a `.tfvar` file:

`main.tf`
```tcl
provider "aws" {
  region     = "eu-west-1"
  access_key = var.access_key
  secret_key = var.secret_key
}
```

`variables.tf`
```tcl
variable "access_key" {
  type      = string
  sensitive = true
}

variable "secret_key" {
  type      = string
  sensitive = true
}
```

`aws.tfvars`
```tcl
access_key = <AWS_ACCESS_KEY_ID>
secret_key = <AWS_SECRET_ACCESS_KEY>
```

Though this gets the credentials into Terraform, I feel icky having these credentials
lying around in plaintext in a file - especially when these credentials usually
have a very wide blast zone, since they're meant to be used to change infrastructure
in a cloud provider. One malformed `.gitignore` entry and a careless `git add .`,
and you could be in for a bad time.

Another common way, that I also use, is with environment variables. For example,
the AWS provider will also look for valid credentials in `AWS_ACCESS_KEY_ID` and
`AWS_SECRET_ACCESS_KEY`, so a typical use of terraform would look like:

```sh
$ export AWS_ACCESS_KEY_ID="anaccesskey"
$ export AWS_SECRET_ACCESS_KEY="asecretkey"
$ terraform plan
```

Doing this every time is tiresome, though. Plus, the credentials are now present
in my shell history file. So my problem was twofold: a consistent,
low-effort, minimal-intervention development environment for terraform without
having the credentials hanging out in plaintext.

## Consistent Environments with Nix Devshells

For a consistent environment, nix's devshells come to the rescue. This way, I can
also fix the terraform version and have a bunch of extra tools, like tflint, without
polluting any other terraform environments I might have. I can also use a
`shellHook` to populate the development environment with the required environment
variables:


`creds.env`
```sh
AWS_ACCESS_KEY_ID="anaccesskey"
AWS_SECRET_ACCESS_KEY="asecretkey"
```

`flake.nix`
```sh
{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.terraform =
          pkgs.mkShell {
            buildInputs = with pkgs; [terraform tflint terraform-docs ];
            shellHook = ''
              set -a
              source ${./creds.env}
              set +a
            '';
          };
      }
    );
}
```

Then, in whatever terraform projects I want these to be in, I can use [direnv](https://direnv.net/),
with an `.envrc` file like:

```
use flake path/to/flake/.#terraform
```

When I drop into a terraform project, it now sets up an environment with a fixed
terraform version, some side-tooling, and an environment populated with the environment
variables I need to work.

However, the credentials are still in plain text, both in the `creds.env` file
next to `flake.nix`, but also in the nix store:

```sh
λ cat /nix/store/y14ih9jfyrqmdxqpg1c5x6aws5162slz-source/creds.env
AWS_ACCESS_KEY_ID="anaccesskey"
AWS_SECRET_ACCESS_KEY="asecretkey"
```

Which is not ideal.

## Secrets Encryption with Mozilla SOPS

I addressed the plaintext credentials with Mozilla's [SOPS](https://github.com/mozilla/sops),
a really nice tool for encrypting, decrypting and editing secrets and credentials.
It works with a variety of filetypes, including dotenv files!

Using my GPG key to encrypt the above `creds.env` file yields the following
(notice that the file itself isn't GPG encrypted, only the individual environment
variables!):

```sh
λ export SOPS_PGP_FP="MY GPG KEY FINGERPRINT"
λ sops --encrypt creds.env
AWS_ACCESS_KEY_ID=ENC[AES256_GCM,data:...,type:str]
AWS_SECRET_ACCESS_KEY=ENC[AES256_GCM,data:...,type:str]
sops_lastmodified=2022-08-27T21:08:17Z
sops_mac=ENC[AES256_GCM,...,type:str]
sops_pgp__list_0__map_fp=MY GPG KEY FINGERPRINT
sops_unencrypted_suffix=_unencrypted
sops_version=3.7.3
sops_pgp__list_0__map_created_at=2022-08-27T21:08:17Z
sops_pgp__list_0__map_enc=-----BEGIN PGP MESSAGE-----\n...\n-----END PGP MESSAGE-----\n
```

I then include this encrypted version of the `creds.env` file with `flake.nix`,
and source the environment variables from it:

```sh
devShells.terraform = pkgs.mkShell {
  buildInputs = with pkgs; [ sops terraform tflint terraform-docs ];
  shellHook = ''
    set -a
    source <(${pkgs.sops}/bin/sops --decrypt ${toString ./creds.env})
    set +a
  '';
};
```

Now every time I drop into terraform environments that need those credentials,
direnv automatically populates my environment with terraform and the associated
credentials (once I unlock the SOPs encrypted file with my GPG passphrase).
Once I leave that directory, the shell is depopulated.
The credentials themselves are no longer in plaintext in a `.tfvars`, `.env` or
shell history file, but are encrypted on disk and decrypted with SOPS and my
GPG key when needed.
