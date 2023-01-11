#include <iostream>
#include <thread>
#include <list>
#include <mutex>
#include <string>

using namespace std;

bool isPrime(int n)
{
    if (n == 2 || n == 3)
        return true;
    if (n <= 1 || n % 2 == 0 || n % 3 == 0)
        return false;
    for (int i = 5; i * i <= n; i += 6)
    {
        if (n % i == 0 || n % (i + 2) == 0)
            return false;
    }
    return true;
}

int main(int argc, char *argv[])
{

    if (argc != 4)
    {
        cout << "Use arguments as follows: first_number, second_number, number_of_threads" << endl;
        return 1;
    }

    int first = stoi(argv[1]), second = stoi(argv[2]);
    int number_of_threads = stoi(argv[3]);
    list<int> primes;
    mutex prime_mutex;
    list<thread> threads;

    for (int i = 0; i < number_of_threads; i++)
    {
        threads.emplace_back(([&first, &second, &number_of_threads, &primes, &prime_mutex, i]
                              {
            for (int j = first + i; j < second; j += number_of_threads) 
            {
                if (isPrime(j))
                {
                    unique_lock<mutex> lock(prime_mutex);
                    primes.emplace_back(j);
                }
            }; }));
    }

    auto threadIt = threads.begin();
    for (int i = 0; i < number_of_threads; i++)
    {
        (*threadIt).join();
        threadIt++;
    }

    auto primeIt = primes.begin();
    for (int i = 0; i < primes.size(); i++)
    {
        cout << (*primeIt) << ", ";
        primeIt++;
    }
    cout << endl;
    cout << "Found " << primes.size() << " prime numbers" << endl;
}