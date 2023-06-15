# seed search mod manager

A backend wrapper for the `SeedSearch` mod for `Slay the Spire`.

## Setup

1. Purchase and install `Slay the Spire` on `Steam`.

- this dumps `Slay the Spire` at a `$STS_HOME` such as `C:\Program Files (x86)\Steam\steamapps\common\SlayTheSpire`.

2. Install `ModTheSpire`
3. Set up [SeedSearch](https://github.com/ForgottenArbiter/SeedSearch). This may require creating `$STS_HOME/mods`, where `SeedSearch.jar` must be found.

## Run

Open `Powershell` at `$STS_HOME`.

Emit output with

```bash
.\jre\bin\javaw.exe -jar ../../workshop/content/646570/1605060445/ModTheSpire.jar --skip-launcher --mods SeedSearch | tee ./path/to/desired/output.txt
```
