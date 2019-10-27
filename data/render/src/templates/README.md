![Logo](awesome.png)

> Static program analysis is the analysis of computer software that is performed without actually executing programs — [Wikipedia](https://en.wikipedia.org/wiki/Static_program_analysis)

This is a collection of static analysis tools and code quality checkers. Pull requests are very welcome!

- :copyright: stands for proprietary software. All other tools are Open Source.
- :warning: indicates that the community does not recommend to use this tool for
  new projects anymore as it is outdated or no longer maintained.

Also check out the sister project, [awesome-dynamic-analysis](https://github.com/mre/awesome-dynamic-analysis).

# Table of Contents

#### [Programming Languages](#programming-languages-1)

<details>
 <summary>Show languages</summary>
  <!-- Please use HTML syntax here so that it works for Github and mkdocs -->
  <ul>
    <li><a href="#abap">ABAP</a></li>
    <li><a href="#ada">Ada</a></li>
    <li><a href="#awk">Awk</a></li>
    <li><a href="#cc">C/C++</a></li>
    <li><a href="#c">C#</a></li>
    <li><a href="#crystal">Crystal</a></li>
    <li><a href="#delphi">Delphi</a></li>
    <li><a href="#dlang">Dlang</a></li>
    <li><a href="#elixir">Elixir</a></li>
    <li><a href="#elm">Elm</a></li>
    <li><a href="#erlang">Erlang</a></li>
    <li><a href="#f">F#</a></li>
    <li><a href="#fortran">Fortran</a></li>
    <li><a href="#go">Go</a></li>
    <li><a href="#groovy">Groovy</a></li>
    <li><a href="#haskell">Haskell</a></li>
    <li><a href="#haxe">Haxe</a></li>
    <li><a href="#java">Java</a></li>
    <li><a href="#javascript">JavaScript</a></li>
    <li><a href="#kotlin">Kotlin</a></li>
    <li><a href="#lua">Lua</a></li>
    <li><a href="#matlab">Matlab</a></li>
    <li><a href="#perl">Perl</a></li>
    <li><a href="#php">PHP</a></li>
    <li><a href="#python">Python</a></li>
    <li><a href="#r">R</a></li>
    <li><a href="#rpg">RPG</a></li>
    <li><a href="#ruby">Ruby</a></li>
    <li><a href="#rust">Rust</a></li>
    <li><a href="#scala">Scala</a></li>
    <li><a href="#shell">Shell</a></li>
    <li><a href="#solidity">Solidity</a></li>
    <li><a href="#sql">SQL</a></li>
    <li><a href="#swift">Swift</a></li>
    <li><a href="#tcl">Tcl</a></li>
    <li><a href="#typescript">TypeScript</a></li>
    <li><a href="#vbscript">VBScript</a></li>
  </ul>
</details>

#### [Multiple languages](#multiple-languages-1)

#### [Other](#other-1)

- [Build tools](#build-tools)
- [Binaries](#binaries)
- [Containers](#containers)
- [Config Files](#config-files)
- [Configuration Management](#configuration-management)
- [CSS](#css)
- [Gherkin](#gherkin)
- [HTML](#html)
- [IDE Plugins](#ide-plugins)
- [LaTeX](#latex)
- [Makefiles](#makefiles)
- [Markdown](#markdown)
- [Mobile](#mobile)
- [Packages](#packages)
- [Supporting Tools](#supporting-tools)
- [Template Languages](#template-languages)
- [Translation](#translation)
- [Web services](#web-services)
- [Writing](#writing)

#### [More Collections](#more-collections-1)

---

# Programming Languages

{% for language, linters in groups.linters %} ## {{ language }}

    {% for linter in linters -%}
    - [{{linter.name }}]({{linter.url | safe }})
    {%- iflinter.proprietary %}:copyright:{% endif %} {% iflinter.deprecated %}:warning:{% endif %} - {{linter.description }}
    {% endfor %}

{% endfor %}

# More collections

{% for collection in groups.collections %}

- [{{ collection.name }}]({{ collection.url | safe }}) - {{ collection.description }}
  {% endfor %}

# License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
Title image [Designed by Freepik](http://www.freepik.com).
