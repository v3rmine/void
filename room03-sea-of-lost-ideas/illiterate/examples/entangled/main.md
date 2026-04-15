# Entangled example directly from their doc

``` {.cpp file=hello_world.cc}
#include <cstdlib>
#include <iostream>

<<example-main-function>>
```

``` {.cpp #hello-world}
std::cout << "Hello, World!" << std::endl;
```

``` {.cpp #example-main-function}
int main(int argc, char **argv)
{
    <<hello-world>>
}
```

``` {.cpp #hello-world}
return EXIT_SUCCESS;
```
