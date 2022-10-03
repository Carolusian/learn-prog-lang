// main.cpp
#include <stdio.h>
#include <iostream>
#include <boost/asio.hpp>

using boost::asio::ip::tcp;

std::string MakeDaytimeString();

// int main(int argc, char *argv[])
int main()
{
    // // Tcp client code
    // if (argc != 2)
    // {
    //     std::cerr << "Usage: client <host>" << std::endl;
    //     return 1;
    // }

    // boost::asio::io_service io_service;
    // tcp::resolver resolver(io_service);

    // tcp::resolver::query query(argv[1], "daytime");
    // tcp::resolver::iterator endpoint_it = resolver.resolve(query);

    // tcp::socket socket(io_service);
    // boost::asio::connect(socket, endpoint_it);

    // while (true)
    // {
    //     std::array<char, 128> buf;
    //     boost::system::error_code error;

    //     size_t len = socket.read_some(boost::asio::buffer(buf), error);

    //     if (error == boost::asio::error::eof)
    //         break;
    //     else if (error)
    //         throw boost::system::system_error(error);

    //     std::cout.write(buf.data(), len);
    // }

    boost::asio::io_service io_service;
    // Deem it as A listener
    tcp::acceptor acceptor{io_service, tcp::endpoint(tcp::v4(), 1337)};

    while (true)
    {
        tcp::socket socket{io_service};
        acceptor.accept(socket);

        std::string message = MakeDaytimeString();
        boost::system::error_code ignored_error;
        boost::asio::write(socket, boost::asio::buffer(message), ignored_error);
    }
    return 0;
}

std::string MakeDaytimeString()
{
    using namespace std;
    time_t now = time(0);
    string timenow = ctime(&now);
    size_t len = timenow.length();

    stringstream ss;
    ss << "HTTP/1.1 200 OK\r\nContent-Length: " << len << "\r\n\r\n"
       << timenow;

    return ss.str();
}