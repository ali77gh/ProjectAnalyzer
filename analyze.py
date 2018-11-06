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
    print("|  > python3 analyze.py 'postfix1'                   |")
    print(" " + 52 * "-")
    print()


def Log(str):
    print("| " + str + ( (51-len(str)) *" ") + "|")


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
        lines+= sum(1 for line in open(i))
    return lines

# main
if(len(sys.argv) == 1):
    ShowHelp()
    exit(0)

if(len(sys.argv) == 3):
    ShowHelp()
    exit()

postfix = sys.argv[1]

Line()
Log("                  ProjectAnalyzer")
Log("")
Log("https://github.com/ali77gh/ProjectAnalyzer")
Log("")
Log("searching...")
files = getFileList(os.getcwd(), postfix)
lines = getLineNumbers(files)
Log("you have " + str(len(files)) + " " + postfix + " files")
Log("you have " + str(lines) + " " + postfix + " lines")
Log("")
Line()
