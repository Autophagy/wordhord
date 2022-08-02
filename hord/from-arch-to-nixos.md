I've found my Arch Linux installations tend to be an accumulation
of packages, quick-fixes and hacks that weigh like a nightmare on my brain.
This is normally not the world's biggest deal, except when I
decide to try and trim down my installed packages and have to try and recall
why I installed X specific package 4 years ago.

It becomes more of a problem when I move between jobs (or just machines within
a job) and I'm suddenly confronted with a blank slate that I, yet again, have to
hammer and quick-fix back into the idiosyncratic shape that I like.

A few years ago, I turned to tools like Ansible to help define what shape I want
my machine to be in a declarative way. This works to some extent - except that I
am lazy. If I have to use a quick-fix or install a package and the choice is either:

1. Install the quick-fix/package
2. Install the quick-fix/package to make sure it works, then encode it in an Ansible script

then I will tend to choose number 1, or more accurately just entirely forget about number 2.
My Ansible definitions ended up being an outdated subset of the configuration of
my machine, a kind of configuration drift familiar to anyone who has to work with IaC tooling
without a regular CI/CD pipeline to enforce it.

A few years ago, on the back of learning Haskell more seriously, I encountered
[Nix](https://github.com/NixOS/nix). Frustrated with "works-on-my-machine" build processes at work with
lots of hidden or implicit dependencies that I had to figure out in order to automate
them, I was quite attracted to Nix's promise about reproducible builds. Nix is essentially
is a language/toolset to define the packaging of a piece of software, which
Nix calls a 'derivation'. Derivations are made up of the specific versioned inputs needed to build
a piece of software - not just the source code, but build dependency tools, packages
stuff like that. These derivations are then built with environments where the only
things allowed into the build environments are the inputs you've specified (which
is why these environments also have no network access).

When learning Nix you quickly come across [NixOS](https://nixos.org/), a Linux flavour based around the
Nix package manager. It extends the declarative focus of Nix to the system itself, letting
me define my system as a Nix expression:

```nix
  hardware.pulseaudio.enable = true;

  hardware.bluetooth = {
    enable = true;
    powerOnBoot = pkgs.lib.mkForce false;
  };

  networking.networkmanager.enable = true;

  time.timeZone = "Europe/Berlin";
  i18n.defaultLocale = "en_GB.UTF-8";

  services.xserver = {
    layout = "us";
    xkbVariant = "";
    enable = true;
    displayManager = {
      lightdm.enable = true;
      defaultSession = "none+i3";
    };
    windowManager.i3.enable = true;

    libinput.enable = true;
  };

  virtualisation.podman = {
    enable = true;
    dockerCompat = true;
  };

  users.users.mika = {
    isNormalUser = true;
    description = "Mika Naylor";
    extraGroups = [ "networkmanager" "wheel" "audio" "video" ];
    shell = pkgs.zsh;
  };
```

The crucial thing about this is that this isn't an extra layer of management on
top of regular methods of changing my system - with NixOS you're more or less forced
to define the system declaratively in order to induce change at all. I appreciate
that this distinction forces me to actually document the changes I make to my
system, the reasons I made it (in the git commit), and gives me the ability to
rebuild the system again if I need to.

Using [Home Manager](https://github.com/nix-community/home-manager) has let me
even bundle my previously ad-hoc dotfile management system into the same Nix
expression format. For example, my ``zsh`` configuration now looks like:

```nix
{
  home.packages = with pkgs; [ pure-prompt ];

  programs.zsh = {
    enable = true;
    dotDir = ".config/zsh";
    enableSyntaxHighlighting = true;
    shellAliases = {
      ll = "ls -l";
      g = "git";
    };
    history = {
      size = 10000;
      path = "$HOME/.zsh_history";
      ignoreSpace = true;
    };
    initExtra = "
      autoload -U promptinit; promptinit
      PURE_PROMPT_SYMBOL="λ"
      prompt pure
    ";
    oh-my-zsh = {
      enable = true;
      plugins = [ "ssh-agent" "git" ];
    };
  };
}
```

Although I'm worried that converting all my dotfiles into this very specific language/form
has a risk of lock-in, having everything defined the same way in the same language
does tickle some part of my monkey brain.

A downside of Nix/NixOS is that learning it can be quite hard. The community points
to the manuals as the resource for learning, but they often read like you need to
already understand the thing they're talking about for them to make sense. If I wasn't
already familiar with some FP concepts, I think I would have been totally lost. The
way I've mostly learned Nix/NixOS has been from spelunking around people's Nix expressions
on Github, which carries the risk of accidentally picking up bad practices or code smells.

I've only been using NixOS full time for a couple months but I'm already quite taken
with it, especially for a work/development machine. The way NixOS works makes me
feel like I have to be fairly principled about the definition of my machine - I
don't do the same thing I did on Arch of installing a package to try it out and
then forgetting that it's on my system for 3 years. With Nix I can try stuff out
with ``nix-shell`` and if I don't decide to make it part of my system, the binaries
get cleaned out on the next nix garbage collection. I'm still keeping Arch on my
personal machine mostly out of pure inertia, but also because I'm a little scared
about getting Steam and all the associated drivers working.

My new, fancy, enflakened NixOS files now live on the main branch of my [Antimber](https://github.com/autophagy/antimber)
repo. I've not run into any blocking problems with it yet as my daily driver,
and picking it up has definitely helped give me the confidence to convert
my projects' existing build processes to Nix Flakes. I'll check back in 6-12 months
down the line to see if I still feel the same way.
