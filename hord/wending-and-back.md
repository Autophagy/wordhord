In 2018, I decided that I had had enough of the Gregorian calendar. It's a phase
that happens to everyone, I think. A entire poem to remember how many days in a
month things are? Month names named after old, dead Roman gods or numbers that
have no actual bearing on the actual month number? Get outta here.

More seriously, the seemingly arbitrary nature of the Gregorian calendar irked
me. It felt like a holdover, bubbling up the same frustrations as when I go home
and have to remember the relationships between miles, yards and feet again versus
centimetres, metres, and kilometres. These frustrations were bubbling up at a time
in my life when I was obsessed with *optimisation* and *rationalisation* of the
processes that I used in my daily life. My time tracking tool, [Færeld](https://github.com/autophagy/faereld)
was a consequence of this - an attempt to track, rationalise and optimise the
time I spent on creative endeavours.

At the time, I was also spending a lot of time around a group of developers, artists
and designers named Merveilles who also had adjacent preoccupations. Inspired
by systems like Devine's [Arvelie](https://wiki.xxiivv.com/site/arvelie.html) calendar,
I decided to try living my life for a little while by a different calendar, that
embodied the values I was looking for.

I already knew of one candidate, of course. I had a calendar of it on my wall as
a gift from a friend and so thought why not try and live by it, instead of it being
a fun political-history novelty - the [French Republican Calendar](https://en.wikipedia.org/wiki/French_Republican_calendar).
I felt this would be fitting, since I sympathised with the republican impulse
to Decimalise All Things, plus had a kind of romantic attraction to the naming
system they deployed.

So, I created the [Wending calendar](https://github.com/autophagy/datarum), and
python library that I could integrate into my personal tooling, modelled on the
republican calendar but with all the names named in their Old English equivalents
to fit with the naming style of my other projects. The structure of my new year
looked like:

| | Old English | Translation | Days |
| -- | -- | -- | -- |
| **Autumn** | Hærfest | Harvest, autumn | 30 |
| | Mist | Mist, fog | 30 |
| | Forst | Frost | 30 |
| **Winter** | Snáw | Snow | 30 |
| | Reg | Rain | 30 |
| | Wind | Wind | 30 |
| | Sǽd | Seed | 30 |
| | Blóstm | Blossom, flower | 30 |
| | Mǽdland | Maedow-land | 30 |
| **Summer** | Rip | Reaping, harvest | 30 |
| | Hát | Heat | 30 |
| | Wæstm | Growth, produce, fruit | 30 |
| | Wending | A turning round, revolution | 5 (6 in leap years) |

The year begins with the Autumn month of Hærfest, on the 22nd of September. Each
month has 30 days, except for the celebratory days at the end of the year, which
can be 5 or 6 depending on whether it is a leap year. In the original republican
calendar, the celebratory days were:

- Celebration of Virtue
- Celebration of Talent
- Celebration of Labour
- Celebration of Opinion
- Celebration of Honours
- Celebration of the Revolution

which was something I always thought was very charming and romantic, similarly
with the names named after seasons, weather and agriculture. Though, of course,
this system absolutely would not work in the southern hemisphere, and frankly
in the middle of Berlin these seasonal-agricultural months had very little
bearing on the actual seasons and whether. Except for Reg and Wind, perhaps.

Everything went great at first - all my tooling had the new calendar integrated
through my Datarum library, my work machine's date display was in the new calendar,
my monthly planning was more regular. Of course, it didn't free me from the
Gregorian calendar at all. If anything, it made me far more aware of it, since the
rest of the world stubbornly refused to adopt my calendar. Appointments, social
events, birthdays, all were still in the old calendar. I had to get good at
memorizing and converting Gregorian dates to Wending, and vice versa, which had
the inconvenient effect of dredging the Gregorian calendar from something that
is generally an invisible, if annoying, part of the fabric of everyday life to
something that had to be constantly paid attention to and worked with. When I
started missing doctor's appointments because of the calendar, I decided I should
wind it down.

It's a pretty obvious conclusion in hindsight, but calendars are social tools.
A calendar in a vacuum isn't very useful, it's real use is from it being a
tool for temporal-social coordination. The effect was humbling and disconcerting.
Trying to go-your-own-way with social coordination systems can be counter-productive,
as the need to interop with the existing system you're trying to break from can
end up heightening your awareness of it in the first place. Even though I can
adopt a better calendar, from my perspective, it still doesn't really matter
since things like calendars aren't decided by their raw utility, but by momentum and
network effects.

Programming languages also occupy this kind of space for me. There are lots of
languages that I think are better than others that I have to use in my day-to-day
life. Outside of my own projects, however, languages are also tools of social
coordination - using a tool no one really knows is hard. Adoption of tools and
languages, especially in professional environments, is a bit of a vicious cycle.
You don't want to adopt a language because its 'unproven' and the ecosystem is small,
so you stick to what you know and whats widely adopted - so you don't adopt the
language, which feeds into it's lack-of-adoption, and on and on. As I've worked
with more teams and in more contexts, I'm more and more aware of the power
momentum and network effects have in tooling choices.

Though sometimes, when I step outside in September, I think to myself "Ah, it's
Hærfest".
