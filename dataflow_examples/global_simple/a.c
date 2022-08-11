#include <pthread.h>

int n = 0;
int x = 0;
int a = 1;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    pthread_mutex_lock(&m);
    n += a;
    x++;
    pthread_mutex_unlock(&m);

    f2();
}

void f2() {
    pthread_mutex_lock(&m);
    n += a;
    pthread_mutex_unlock(&m);

    x++;

    f3();
}

void f3() {
    pthread_mutex_lock(&m);
    n += a;
    pthread_mutex_unlock(&m);
}

void *t_fun(void *arg) {
    f1();
    return NULL;
}

int main() {
    pthread_t id;
    pthread_create(&id, NULL, t_fun, NULL);
    pthread_join(id, NULL);
}
