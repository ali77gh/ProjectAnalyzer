import os;

formatList = []

class Data :
    def __init__(self , formats):
        self.keys = format
        self.values = []
        print("data object created on RAM")

def getDirContent():
    return os.listdir(os.getcwd())

def getLineNumber(fileName):
    fp = open(fileName, 'r')  
    line = fp.readline()
    cnt = 0
    while line:
        line = fp.readline()
        cnt += 1
    return cnt

# print(30*"-")
# print("enter postfix of file  ")
# print("example -> py")
# print("enter con to continue")
# print(30*"-")

# exitLoop = False
# while True:
#     newformat = input("enter postfix : ")
#     if newformat=="con":
#         break
#     formatList.append(newformat)

# print(formatList)

# rootContext = getDirContent()
# print(rootContext)

# for i in rootContext:
#     try:
#         cnt= getLineNumber(i)
#         print(cnt)
#     except:
#         pass

data =Data()
