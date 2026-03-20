x = int(input("Enter dividend: "))
ans = []
while x > 0:
    x, r = divmod(x, 2)
    ans.append(r)
ans.reverse()
print(ans)