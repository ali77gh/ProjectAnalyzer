#!/usr/bin/python3

from fnmatch import filter
import os
import sys


def ShowHelp():
    print()
    print("┌" + (62 * "─") + "┐")
    print("│                       ProjectAnalyzer                        │")
    print("│                                                              │")
    print("│ github:https://github.com/ali77gh/ProjectAnalyzer            │")
    print("│                                                              │")
    print("│ how to use :                                                 │")
    print("│  > python3 analyze.py 'postfix'                              │")
    print("│  > python3 analyze.py 'postfix' --ignore 'dir or files names'│")
    print("└" + (62 * "─") + "┘")
    print()


def ShowInBox(str):
    print("│ " + str + ((51-len(str)) * " ") + "│")


def TopLine():
    print("┌" + (52 * "─") + "┐")

def BottomLine():
    print("└" + (52 * "─") + "┘")

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



def RemoveIgnores(allFiles, ignores):
    for i in allFiles:
        for j in ignores:
            if i.find(j)>0:
                allFiles.remove(i)


def ValidateInputs():
    firstMode = len(sys.argv) == 2 and (not sys.argv[1] == "--ignore")
    secondMode = len(sys.argv) > 3 and sys.argv[2] == "--ignore"
    if (not firstMode) and (not secondMode):
        print("bad way to use")
        ShowHelp()
        exit(0)

# main
if len(sys.argv) == 1 or sys.argv[1] == "--help" :
    ShowHelp()
    exit(0)

ValidateInputs()

ignores = []
if len(sys.argv) > 3 and sys.argv[2] == "--ignore":
    ignores = sys.argv[3:]

postfix = sys.argv[1]

TopLine()
ShowInBox("                  ProjectAnalyzer")
ShowInBox("")
ShowInBox("https://github.com/ali77gh/ProjectAnalyzer")
ShowInBox("")
ShowInBox("searching...")
files = getFileList(os.getcwd(), postfix)
RemoveIgnores(files, ignores)
if len(files) == 0:
    ShowInBox("there is no " + postfix + " file")
    BottomLine()
    exit()

try:
    lines = getLineNumbers(files)
except UnicodeDecodeError as e:
    ShowInBox("this file type is not unicode :|")
    BottomLine()
    exit()

linePerFile = round(lines / len(files), 2)
ShowInBox("you have " + str(len(files)) + " " + postfix + " files")
ShowInBox("you have " + str(lines) + " " + " lines of " + postfix)
ShowInBox("lines per file average: " + str(linePerFile))
ShowInBox("")
BottomLine()

# todo --ignore param
