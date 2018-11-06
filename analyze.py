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


# main
if(len(sys.argv) == 1):
    ShowHelp()
    exit(0)

if(len(sys.argv) == 3):
    ShowHelp()
    exit()

postfix = sys.argv[1]

files = getFileList(os.getcwd(), postfix)

Line()
Log("")
Log(postfix + " files count : " + str(len(files)))
Log("")
Line()
