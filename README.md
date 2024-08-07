# Infoskjerm for Trondheimkontoret

Infoskjerm for Trondheimkontoret, skrive i [Rust](https://www.rust-lang.org/), med [Slint](https://slint.dev/).

<img src="screenshot.png" width="500"/>

## Kom i gang!
* [Installer Rust](https://www.rust-lang.org/tools/install)
* Installer [Rust Analyzer for VSCode](https://code.visualstudio.com/docs/languages/rust)*
* Installer [Slint plugin for VSCode](https://marketplace.visualstudio.com/items?itemName=Slint.slint) *
* `cargo run` bygger og starter applikasjonen

    <sub>*[Rust Analyzer](https://rust-analyzer.github.io/manual.html) og [Slint plugin](https://slint.dev/get-started#integrate-with-ides) er også støtta av andre editors.</sub>

Kildekoden ligg i [src-mappa](./src). `main.rs` inneheld entrypoint for applikasjonen. Undermapper med ein `mod.rs-fil` er moduler for ulik funksjonalitet.


GUIet er laga med DSL'en Slint, og ligg i ui-mappa. [mainwindow.slint](./ui/mainwindow.slint) eksponerar `in properties` som blir tilgjengelige fra Rust-koden med setters.   F.eks:  
`in property <string> month: "jan";` eksponerar `set_month(string: SharedString)`, og har "jan" som defaultverdi.

SharedString er ein spesiell type String brukt av Slint, og lagast ved å kalle `.into()` på `&str` og `Strings`. Strengar er litt rare i Rust. Sjå gjerne [denne](https://doc.rust-lang.org/rust-by-example/std/str.html).

GUI-koden eksponerar også typar, som `Forecast`-structen i [weatherwidget.slint](./ui/forecastwidget.slint). Denne blir også tilgjengelig i Rust-koden gjennom `ui`-modulen.



## Implementert funksjonalitet
* XKCD
* Vær
* Klokke og dato
* Matsporing via Wolt
* Busstider
* Aktivitetskalender

## Idear til funksjonalitet
* Møteromsstatus
* Matsporing fra Foodora
* AI assistent: For *GPT med data og kontekst, dato, kalender, vær etc etc og be den gi råd for morgendagen eller lignende.
* Bursdager 🎉
* Dagens visdomsord/quote
* Nyhende / overskrifter
* Countdown til neste seminar
* Andre comics?

## Andre tanker
* Koderydding
* Feilhåndtering
* Fikse minnelekkasje?

## Vedlikehold

### Oppdater skjema-definisjoner fra EnTurs API.

```shell
graphql-client introspect-schema "https://api.entur.io/journey-planner/v3/graphql" --output "src/resources/journey-planner_schema.json"
```
