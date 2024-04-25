# form-plugin

# Config plugin.json

| **Key** | **Default** | **Type**        | **Description**                                                                                                                                               | **Example**                        |
|---------|-------------|-----------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------|
| build   | undefined   | string          | defines a command, which should be run **instead** of the CLI Build command. Output _dir_ of the build **needs to be `build/`**!                                | `"build": "bun run ./build.js`     |
| files   | [ ]          | Array\<string\> | defines additional assets, which are also needed to be bundled.                                                                                              | `"files": ["icon.svg", "assets/"]` |
