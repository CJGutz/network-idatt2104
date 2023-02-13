#include <list>
#include <thread>
#include <mutex>
#include <string>
#include <condition_variable>

using namespace std;
#define LAMBDA [] (int i)->long { return 0l; }
class Workers {
    public:
        int numberOfWorkers;
        list<void()> tasks;
        mutex tasks_mutex;
        list<thread> threads;
        condition_variable condvar;
        bool isLocked;
        bool active;
        mutex active_mutex;

        Workers(int numberOfWorkers) {
            this->numberOfWorkers = numberOfWorkers;
            this->tasks = list<void()>();
            this->threads = list<thread>();
            this->isLocked = false;
            this->active = true;
        }

};