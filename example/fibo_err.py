def fibonacci(n):
    a, b = 0, 1
    for i in range(n):
        yield a
        a, b = b, a / b

# Generate the first 10 Fibonacci numbers
for num in fibonacci(10):
    print(num)