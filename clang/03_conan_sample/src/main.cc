#include <iostream>
#include <fmt/format.h>

int main(int argc, char **argv)
{
    std::string s = fmt::format("{:d}", 10);
    std::cout << s << std::endl;
    return 0;
}
