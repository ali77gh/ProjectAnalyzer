# ProjectAnalyzer
this script can analyze your project (count your files and how mach line of code you write)

## how to use
1. get analyzer.py
2. put analyzer.py in root of your project 
3. run:
~~~posh
python3 analyzer.py <postfix>
~~~
or
~~~posh
python3 analyzer.py <postfix> --ignore <ignore paths...>
~~~
4. see magic :|

## example
normal mode
~~~posh
$ python3 analyze.py py
 ----------------------------------------------------
|                   ProjectAnalyzer                  |
|                                                    |
| https://github.com/ali77gh/ProjectAnalyzer         |
|                                                    |
| searching...                                       |
| you have 1 py files                                |
| you have 103  lines of py                          |
| lines per file avrage: 103.0                       |
|                                                    |
 ----------------------------------------------------
~~~
ignore mode
~~~posh
$ python3 analyze.py py --ignore aaa bbb ccc
 ----------------------------------------------------
|                   ProjectAnalyzer                  |
|                                                    |
| https://github.com/ali77gh/ProjectAnalyzer         |
|                                                    |
| searching...                                       |
| you have 1 py files                                |
| you have 103  lines of py                          |
| lines per file avrage: 103.0                       |
|                                                    |
 ----------------------------------------------------
~~~