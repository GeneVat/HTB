# HTB (HTML to BBCode)

HTB is a command-line tool that converts a subset of HTML into BBCode, suitable for use on nationstates.

To start, run `npm run start` with Node.js installed and then edit the `main.html` file.

## Tags

Below is a list of supported HTML tags and their BBCode equivalents. Use these tags in your `main.html` file.

| HTML Tag(s)         | BBCode Equivalent         | Example Usage (HTML)                   | Resulting BBCode                |
|---------------------|--------------------------|----------------------------------------|---------------------------------|
| `<float>`           | `[float]`                | `<float>Text</float>`                  | `[float]Text[/float]`           |
| `<align>`           | `[align]`                | `<align>Text</align>`                  | `[align]Text[/align]`           |
| `<tab>`             | `[tab]`                  | `<tab>Indented</tab>`                  | `[tab]Indented[/tab]`           |
| `<box>`             | `[box]`                  | `<box>Boxed</box>`                     | `[box]Boxed[/box]`              |
| `<sidebar>`         | `[sidebar]`              | `<sidebar>Side</sidebar>`              | `[sidebar]Side[/sidebar]`       |
| `<blockquote>`      | `[sidebar]`              | `<blockquote>Side</blockquote>`        | `[sidebar]Side[/sidebar]`       |
| `<b>`, `<strong>`   | `[b]`                    | `<b>Bold</b>`                          | `[b]Bold[/b]`                   |
| `<i>`, `<em>`       | `[i]`                    | `<i>Italic</i>`                        | `[i]Italic[/i]`                 |
| `<u>`               | `[u]`                    | `<u>Underline</u>`                     | `[u]Underline[/u]`              |
| `<sup>`             | `[sup]`                  | `<sup>Sup</sup>`                       | `[sup]Sup[/sup]`                |
| `<sub>`             | `[sub]`                  | `<sub>Sub</sub>`                       | `[sub]Sub[/sub]`                |
| `<strike>`, `<del>`, `<s>` | `[strike]`        | `<s>Strike</s>`                        | `[strike]Strike[/strike]`       |
| `<pre>`             | `[pre]`                  | `<pre>Code</pre>`                      | `[pre]Code[/pre]`               |
| `<code>`            | `[code]`                 | `<code>Code</code>`                    | `[code]Code[/code]`             |
| `<hr>`              | `[hr]`                   | `<hr>`                                 | `[hr]`                          |
| `<a id="URL">`    | `[url=URL]`              | `<a id="https://x.com">X</a>`        | `[url=https://x.com]X[/url]`    |
| `<anchor id="foo">` | `[anchor=foo]`           | `<anchor id="foo">Jump</anchor>`       | `[anchor=foo]Jump[/anchor]`     |
| `<nation>`          | `[nation]`               | `<nation>Example</nation>`             | `[nation]Example[/nation]`      |
| `<region>`          | `[region]`               | `<region>Region</region>`              | `[region]Region[/region]`       |
| `<proposal>`        | `[proposal]`             | `<proposal>Prop</proposal>`            | `[proposal]Prop[/proposal]`     |
| `<resolution>`      | `[resolution]`           | `<resolution>Res</resolution>`         | `[resolution]Res[/resolution]`  |
| `<quote>`           | `[quote]`                | `<quote>Quote</quote>`                 | `[quote]Quote[/quote]`          |
| `<h1>`              | `[size=150]`             | `<h1>Big</h1>`                         | `[size=150]Big[/size]`          |
| `<h2>`              | `[size=135]`             | `<h2>Header</h2>`                      | `[size=135]Header[/size]`       |
| `<h3>`              | `[size=120]`             | `<h3>Header</h3>`                      | `[size=120]Header[/size]`       |
| `<h4>`              | `[size=105]`             | `<h4>Header</h4>`                      | `[size=105]Header[/size]`       |
| `<h5>`              | `[size=90]`              | `<h5>Header</h5>`                      | `[size=90]Header[/size]`        |
| `<p>`               | `[size=75]`              | `<p>Paragraph</p>`                     | `[size=75]Paragraph[/size]`     |
| `<color>`           | `[color]`                | `<color>Red</color>`                   | `[color]Red[/color]`            |
| `<background>`      | `[background]`           | `<background>BG</background>`          | `[background]BG[/background]`   |
| `<background-block>`| `[background-block]`     | `<background-block>BG</background-block>` | `[background-block]BG[/background-block]` |
| `<spoiler>`         | `[spoiler]`              | `<spoiler>Hidden</spoiler>`            | `[spoiler]Hidden[/spoiler]`     |
| `<ul>`, `<ol>`      | `[list]`                 | `<ul><li>Item</li></ul>`               | `[list][*]Item[/list]`          |
| `<li>`              | `[*]`                    | `<li>Item</li>`                        | `[*]Item`                       |
| `<table>`           | `[table]`                | `<table>...</table>`                   | `[table]...[/table]`            |
| `<tr>`              | `[tr]`                   | `<tr>...</tr>`                         | `[tr]...[/tr]`                  |
| `<td>`, `<th>`      | `[td]`                   | `<td>Cell</td>`                        | `[td]Cell[/td]`                 |

- **ID Attributes:** If you use `id="foo"` on a tag, it will be converted to `[tag=foo]`. This is how to modify tags.
