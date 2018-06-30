# i write all codes on 1 file becuse its easy to move for users ;)
import os

allFiles = []
fileTypes = []
debug = 1  # 1: enable logs


class FileType:
    # dotSth is string like -> "js" , "py" ,...
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
            LogIt(self.key + " files num:" + self.filesNum + " lines: " + self.linesNum)

    def safeAddFiles(self, filePathsList):
        for i in filePathsList:
            self.safeAddFile(i)


def getDirContent():
    return os.listdir(os.getcwd())


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


print(30*"-")
print("postfix like -> py , js , java , cpp ...")
print("enter con to continue")
print(30*"-")

#TODO : get dot sths as argmants if user do not enter run blow code
#read dot sth's
exitLoop = False
while True:
    newformat = input("enter postfix : ")
    if newformat == "con":
        break
    fileTypes.append(FileType(newformat))

rootContext = getDirContent()
print(rootContext)

for i in rootContext:
    try:
        cnt = getLineNumber(i)
        print(cnt)
    except:
        pass

d = FileType("js")
