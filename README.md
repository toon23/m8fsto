# m8fsto

Command line helper to manage M8 files

Available command :

 * `ls-sample` : to list samples used in a song file along with its instrument number
 * `grep-sample` : to find song using a specific sample
 * `broken-search` : Find songs with missing samples
 * `bundle` : Bundling a song (without a M8)

## Examples

### ls-sample

```
> m8fsto ls-sample 'C:\Users\twins\tracks\M8 backup\Songs\UNFINISHED\AMCHORD.m8s'

C:\Users\twins\tracks\M8 backup\Songs\UNFINISHED\AMCHORD.m8s
  00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
  01 SNARECKK : /Samples/Drums/Hits/TR909/SD/ST0T0S7.wav
  02 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
  03 : /Samples/Drums/Hits/TR909/CH/HHCD4.wav
  04 : /Samples/Drums/Hits/TR909/CR/RIDED6.wav
```

This example will list the samples used in a specific M8 song.
The number is the instrument number, with the optional instrument
name, followed by the sample path.


```
m8fsto ls-sample 'C:\Users\twins\tracks\M8 backup/**/*.m8s'
```

Will display the list of all samples used in songs present in a backup folder
or SD card.

### grep-sample

A reverse proposition from ls-sample, we have a sample, but we want to find
the song using them, so we grep-sample to search through the songs.

```
> m8fsto grep-sample '*/BT7AADA.wav' 'C:\Users\twins\tracks\M8 backup\Songs\**/*.m8s'

C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\04 APRIL\DADRO.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\06 JUNE\VIAKE.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\DABASSTE.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\FSTUB.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\FSTUB.m8s:01 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\GNARTTERY.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\HYJUNGLE.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\IDEABOX.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\IDEABOX.m8s:01 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\11-NOV\SADUB.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\12-DEC\DABASSTE.m8s:00 909KICKK : /Samples/Drums/Hits/TR909/BD/BT7AADA.wav
```

Here we want to search for the usage of a 909 kick sample through all the songs.
The pattern AND the folder use glob matching. We get the song file, the instrument
number and name, along with the full matched sample.

```
> m8fsto grep-sample '**/SFM/*' 'C:\Users\twins\tracks\M8 backup\Songs\**/*.m8s'

C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\GNARTTERY.m8s:01 909KICKK : /Samples/Packs/SFM/essen/Drums/02. Kits/CR78 From Mars/02. Kit 2/Conga CR78 01.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\GNARTTERY.m8s:10  : /Samples/Packs/SFM/essen/Drums/02. Kits/DMX From Mars/08. 612 Echo/SD DMX 612 Echo 11.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\GNARTTERY.m8s:11  : /Samples/Packs/SFM/essen/Drums/02. Kits/CR78 From Mars/02. Kit 2/OH CR78 06.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\IDEABOX.m8s:02  : /Samples/Packs/SFM/909/Individual Hits/06. Hi Hat/01. CH/02. Color/CH 909 Color 02.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\IDEABOX.m8s:10  : /Samples/Packs/SFM/909/Individual Hits/05. Hand Clap/02. Color/Clap 909 Color 02.wav
C:\Users\twins\tracks\M8 backup\Songs\BOF\2024\10-OCT\IDEABOX.m8s:60  : /Samples/Packs/SFM/essen/Synths/Dr Sample From Mars/Bass/Goon Bass Dr Sample 13 C1.wav
```

This command for the usage of any sample within `SFM` folder

### broken-search

Broken search will list songs using sample that has been moved or
removed, and can no longer be found in your backup or SD card.

```
m8fsto broken-search 'C:\Users\twins\tracks\M8 backup'
```

will search for all of the broken songs present in a M8 sd
card backup (or directly on the SD card if you want).
