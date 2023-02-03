// __declspec(noinline) ensures we won't inline this function
int __declspec(noinline) add(int a, int b) {
    return a + b;
}

#pragma optimize("", off) // do not optimize away the call to "add"
int main(int argC, char **argV) {
    return add(1234, 5678);
}