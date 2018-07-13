# i write all codes on 1 file becuse its easy to move for users ;)
import os

allFiles = []
fileTypes = []
debug = 1  # 1: enable logs , 0:disable logs


def getPostfixExplanation(postfix):
    if postfix == "js":
        return "javascript file"
    elif postfix == "py":
        return "python file"
    elif postfix == "java":
        return "java file"
    elif postfix == "sh":
        return "bash script"
    elif postfix == "jar":
        return "jar package"
    elif postfix == "xml":
        return "extensible markup (xml)"
    elif postfix == "png":
        return "bitmap (png)"
    elif postfix == "jpg":
        return "bitmap (jpg)"
    elif postfix == "svg":
        return "vector (svg)"
    elif postfix == "cs":
        return "c# file"
    elif postfix == "ts":
        return "typescriipt"
    elif postfix == "rb":
        return "ruby file"
    elif postfix == "htm" or postfix == "html":
        return "hyper text markup file (html)"
    elif postfix == "css":
        return "style sheet"
    elif postfix == "txt" or postfix == "text":
        return "text file"
    elif postfix == "json":
        return "json file"

    #not available
    return postfix + " file"


class FileType:
    # dotSth is string like -> "js" , "py" , "cs"...
    def __init__(self, dotSth):
        self.key = dotSth
        self.filesNum = 0  # how match files
        self.linesNum = 0  # how match lines
        self.linesAvg = 0  # TODO average lines of code in one file
        LogIt("FileType " + dotSth + "object created")

    # returns bool
    def isKeyMatch(self, nkey):
        return self.key == nkey

    def getFilePathDotSth(self, filePath):
        nothing, dotSth = filePath.split(".")
        return dotSth

    def safeAddFile(self, filePath):
        # check dot sth
        dotSth = self.getFilePathDotSth(filePath)
        if not self.isKeyMatch(dotSth):
            return
        else:
            # is match
            self.filesNum += 1
            self.linesNum = getLineNumber(filePath)
            LogIt(self.key + " files num:" +
                  self.filesNum + " lines: " + self.linesNum)

    def safeAddFiles(self, filePathsList):
        for i in filePathsList:
            self.safeAddFile(i)


def getDirContent(subPath):
    if subPath == "":
        return os.listdir(os.getcwd())
    else:
        return os.listdir(os.getcwd() + "/" + subPath)


def LogIt(message):
    if debug:
        print("Log : -> ", message)


def getLineNumber(fileName):
    fp = open(fileName, 'r')
    line = fp.readline()
    cnt = 0
    while line:
        line = fp.readline()
        cnt += 1
    return cnt


def ShowHelp():
    print()
    print(" " + 52 * "-")
    print("|                 ProjectAnalyzer                    |")
    print("|                                                    |")
    print("| github:https://github.com/ali77gh/ProjectAnalyzer  |")
    print("|                                                    |")
    print("| for specific file :                                |")
    print("|  > python3 analyze.py 'postfix1' 'postfix2' ,...   |")
    print(" " + 52 * "-")
    print()


def getPostFixs():
    while True:
        newformat = input("> ")
        if newformat == "con":
            break
        fileTypes.append(FileType(newformat))


def getProjectAllFilesAndDirs(rootContent):
    while 1:
        isSthChange = 0
        for i in rootContent:
            try:
                # dir
                newDirs = getDirContent(i)
                allFiles.extend(newDirs)
                isSthChange = 1
            except:
                # file
                allFiles.append(i)
        if isSthChange == 0:
            break

# TODO : get dot sths as argmants if user do not enter run blow code


# main
ShowHelp()
# getPostFixs()

#rootDir = getDirContent("")

# print(getProjectAllFilesAndDirs(rootDir))

# ------------------------todo-------------------------
# 1.input                             deleted         |
# 2.make files list                                   |
# 3.pass list to FileTypes                            |
# 4.get report from FileTypes                         |
# -----------------------------------------------------
