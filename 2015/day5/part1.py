import hashlib

resultingHash = ''

i = 0

while not resultingHash.startswith('000000'):
    i += 1
    prep = ('yzbqklnj' + str(i)).encode('utf-8')
    resultingHash = hashlib.md5(prep).hexdigest()

print(resultingHash)
print(i)
