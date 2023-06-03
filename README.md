# ProjectAnalyzer

Simple script that counts number of files and lines of code that you write in your project.

## Installaion

```sh
curl -sfL https://raw.githubusercontent.com/ali77gh/ProjectAnalyzer/master/install.sh | sudo bash -
```

## How to use

~~~posh
analyzer.py <postfix>

┌────────────────────────────────────────────────────┐
│                   ProjectAnalyzer                  │
│                                                    │
│ https://github.com/ali77gh/ProjectAnalyzer         │
│                                                    │
│ searching...                                       │
│ you have 1 py files                                │
│ you have 104  lines of py                          │
│ lines per file average: 104.0                      │
│                                                    │
└────────────────────────────────────────────────────┘
~~~

## Ignore switch
~~~posh
python3 analyzer.py py --ignore dir1 dir2 file1 file2
~~~
