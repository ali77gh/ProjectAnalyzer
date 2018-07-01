# i write all codes on 1 file becuse its easy to move for users ;)
import os

allFiles = []
fileTypes = []
debug = 1  # 1: enable logs , 0:disable logs

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
        return  os.listdir(os.getcwd())
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
    print(" " + 50 * "-")
    print("|                       HELP                       |")
    print("| enter postfix like -> py , js , java , cpp ...   |")
    print("| enter 'con' to continue                          |")
    print(" " + 50 * "-")
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
                #dir
                newDirs = getDirContent(i)
                allFiles.extend(newDirs)
                isSthChange = 1
            except:
                #file
                allFiles.append(i)
        if isSthChange == 0:
            break

# TODO : get dot sths as argmants if user do not enter run blow code

#main 
ShowHelp()
getPostFixs()

rootDir = getDirContent("")

print(getProjectAllFilesAndDirs(rootDir))

#-----------------------------------------------------
# 1.input                               check         |
# 2.make files list                                   |
# 3.pass list to FileTypes                            |
# 4.get report from FileTypes                         |
#-----------------------------------------------------