# To Do

- change in "edit projects" implementieren
- Settings einbauen
- > manual input -> struct anpassen
- > notifications -> struct anpassen
- > logging
- ssh einbauen -> struct anpassen
- file-picker exe ins projekt packen

# Ideas

- Automatische übertragung über ftp oder ssh? vielleicht nutzlos
- NAME: **File Moth**
- Die System Notes bekommen als Image einen Mottenkopf!!!
- **Settings:** (Notifications disable), (enable manual input/disable file picker)
- Verschlüsselung iwie unterstützen maybe
- passkey für einlogen ins programm
- **archiv mode:** es werden komprimierte datein gesichert + option das mehrere snapshots gespeichert werden
- Magiv wormhole crate: https://docs.rs/magic-wormhole/latest/magic_wormhole/
- > besser geeignet für übertragung von datein

# Knowledge

- what is Serealization: https://medium.com/@murataslan1/serialization-and-deserialization-in-rust-a-comprehensive-guide-3eb249ae8ac6

# Dameon mode

Die config File sollte wahrscheinlich am besten konstant in den heap geladen werden und nicht jede minute beim check erneut.

Die Optionen könnten so aussehen:
**There are multiple options how to use the daemon mode**
stupid:

- when directory is modified
- Safe x times / day (max. 24)
- Safe x times / week (max. 7)
- Safe x times / month (max. 10)
- Safe at XX:XX every day
- Safe every Weekday x at XX:XX
- Safe every XX.XX. of the Month at the time XX:XX

besser:

- safe when directory is modified
- safe once a day (time optional)
- safe once a week (weekday and time optional)
- safe once a month (day and time optional)

## Daemon main arrays

Damit das program weiß wann es schauen muss welche projekte gesichert werden, wird ein array geführt, in dem die sich ergebenden speicherzeiten gespeichert werden.

1. Daytimes: [00:00, 06:00, 12:00, 18:00]
2. Weekdays: [2, 3, 7]
3. Monthdays: [1, 24, 30]

- Es gibt also einen array der alle speciherzeiten speichert.
- Die arrays werden numerisch sortiert
- Einmal in der Minute wird die aktuelle Zeit und Wochentag mit den Zeiten/Tagen abgeglichen
- Es wird nur bis zu der ersten Zeit/ dem ersten Tag gelesen der noch nicht eingetreten ist
- > mit oberem beispiel: Wenn es gerade 08:34 ist wird bis 12:00 Uhr der Array verglichen.
- > wenn festgestellt wird Zeit/Tag im Array ist größer wird for-loop beendet
- Wenn Übereinstimmung -> Die korrespondierenden Felder im project array werden für alle projekte gelesen um die relevanten Projekte zu finden

- es muss für jedes daemon projekt gespeichert werden, ob der safe in der aktuellen speicher periode schon durchgeführt wurde

## Daemon project Array

Ein Array der für jedes Projekt erzeugt wird und die folgenden Felder enthält. Es können mehrere zeiten pro projekt gespeichert werden:

1. [1/0] = gibt an ob daemon verwendet wird (1) oder nicht (0)
2. [(time),(time), ...] = Tageszeit also z.B.: 13:00, 12:25
3. [((weekday),(time)), ((weekday),(time)), ...] = Wochentag und Zeit an dem Projekt gespeichert wird. Z.B. [((5),(07:30)), ((7),(09:30))]
4. [((day),(time)), ((day),(time)), ...] = Monatstag und Zeit an dem Projekt gespeichert wird. Z.B. [((24),(07:30)), ((1),(19:30))]
5. [timestamp last safe] = Der Zeitpunkt wann das projekt das letzte mal gespeichert wurde. damit kann überprüft werden, ob safe verpasst wurde während computer aus war

-> Für den monatlichen safe muss bei Tag > 28 untersucht werden wie viele Tage der monat hat. Wenn Tag > Monat muss das projekt vorzeitig gespeichert werden

# Log & Config Files

- Für alles OS muss ich passende Ordner finden wo die Logs und die Config hinkommen

**Mac**

- https://stackoverflow.com/questions/15677388/default-location-for-configuration-files-macos
- https://developer.apple.com/library/archive/documentation/General/Conceptual/MOSXAppProgrammingGuide/AppRuntime/AppRuntime.html

# Homebrew

- nicht unbedingt notwendig aber wär fresh wenns app da gibt
- Anleitung: https://docs.brew.sh/Adding-Software-to-Homebrew

# Tailscale

- zum kopieren von Datein wird **Taildrop** genutzt
- > https://tailscale.com/docs/features/taildrop
- > Cli nutzung erklärt: https://tailscale.com/docs/reference/tailscale-cli#file
- im Cli wird eine Datei zu einem Gerät im Tailnet wie folgt geschickt:

```bash
$ tailscale file cp <files...> <target>:

$ tailscale file cp fuck.png ya-computa:
```

- Die Datein kommen auf dem Zielgerät (hier "ya-computa") im Download ordner an
- wenn man will das die ankommenden Datein wo anders hinkommen kann man folgendes nutzen:

```Bash
$ tailscale file get <target-directory>

$ tailscale file get /Users/ziegenhals7000/Movies --loop
```

- die Flag "--loop" sorgt dafür das kontinuierlich geschaut wird ob was reinkommt
- oder "--wait" auch so ähnlich villeicht auch besser

- man kann die Gerätenamen und ip's im Tailnet zu den man schicken kann auflisten lassen:

```bash
$ tailscale file cp --targets
```

## tailscale directories schicken

- es können nur einzelne datein von tailscale verschickt werden
- > wenn ordner geschickt werden soll muss er als zip oder tar geschickt werden
- dazu kommt natürlich, dass immer der gesamte ordner geschickt werden muss
- **user recommendation:**
- die handhabung der gesendeten ordner/datein sollte am Zielcomputer manuell eingestellt werden
- datein landen in _Downloads_ können aber von tailscal verschoben werden
- ebenfalls kann eingestellt werden, dass immer nur die neuste version gespeichert wird und alte gelöscht werden


# Custom safe commands

- man könnte im dest Vektor auch ein custom commands speichern 
- user könnten einen eigenen safe command nutzen um projekte auf eigenem weg zu speichern

# Destination Vektor

Der aufbau vom Destination Vektor ist:

Vec<String> = [safe-mode, Ziel, bundle/compression-mode] 

-> custom command könnte noch dazu kommen
