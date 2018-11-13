#!usr/bin/python3
# i write all codes on 1 file because its easy to move for users ;)

from fnmatch import filter
import os
import sys


def ShowHelp():
    print()
    print(" " + 52 * "-")
    print("|                 ProjectAnalyzer                    |")
    print("|                                                    |")
    print("| github:https://github.com/ali77gh/ProjectAnalyzer  |")
    print("|                                                    |")
    print("| how to use :                                       |")
    print("|  > python3 analyze.py 'postfix'                    |")
    print(" " + 52 * "-")
    print()


def ShowInBox(str):
    print("| " + str + ((51-len(str)) * " ") + "|")


def Line():
    print(" " + 52 * "-")


def getFileList(path, filePostfix):
    array = []
    file_name = "*."+filePostfix
    for root, dirs, files in os.walk(path):
        for filename in filter(files, file_name):
            array.append(os.path.join(root, filename))
    return array


def getLineNumbers(Files):
    lines = 0
    for i in Files:
        lines += sum(1 for line in open(i))
    return lines


# main
if(len(sys.argv) == 1):
    ShowHelp()
    exit(0)

if(len(sys.argv) > 2):
    ShowHelp()
    exit()

if(sys.argv[1]=="--help"):
    ShowHelp()
    exit()

postfix = sys.argv[1]

Line()
ShowInBox("                  ProjectAnalyzer")
ShowInBox("")
ShowInBox("https://github.com/ali77gh/ProjectAnalyzer")
ShowInBox("")
ShowInBox("searching...")
files = getFileList(os.getcwd(), postfix)
if len(files) == 0:
    ShowInBox("there is no "+ postfix + " file")
    Line()
    exit()
    
try:
    lines = getLineNumbers(files)
except UnicodeDecodeError as e:
    ShowInBox("this file type is not unicode :|")
    Line()
    exit()

linePerFile = round(lines / len(files), 2)
ShowInBox("you have " + str(len(files)) + " " + postfix + " files")
ShowInBox("you have " + str(lines) + " " + " lines of " + postfix)
ShowInBox("lines per file avrage: " + str(linePerFile))
ShowInBox("")
Line()

# todo --ignore param
