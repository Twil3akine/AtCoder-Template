n = int(input())
p = list(map(int, input().split()))
pp = [0]*n
for i in range(n):
	pp[p[i]-1] = i
	
print(pp)

l = [-1] * n

for i in range(n-1):
	
