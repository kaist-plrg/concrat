#include <pthread.h>

int n1 = 0;
int n2 = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    pthread_mutex_lock(&m);
    a1();
    b1();
    pthread_mutex_unlock(&m);

    pthread_mutex_lock(&m);
    a1();
    b1();
    pthread_mutex_unlock(&m);
}

void f2() {
    pthread_mutex_lock(&m);
    a1();
    pthread_mutex_unlock(&m);

    b1();
}

void a1() {
    n1++;
    a2();
}

void a2() {
    n1++;
}

void b1() {
    n2++;
    b2();
}

void b2() {
    n2++;
}

int main() {}
