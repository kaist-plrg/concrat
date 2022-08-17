#include <pthread.h>

int n = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    int x;

    pthread_mutex_lock(&m);
    n = n + 1;
    pthread_mutex_unlock(&m);

    x = pthread_mutex_lock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }
}

void f2() {
    int x;

    pthread_mutex_lock(&m);
    n = n + 1;
    pthread_mutex_unlock(&m);

    x = pthread_mutex_trylock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }
}

void f3() {
    int x;

    pthread_mutex_lock(&m);
    n = n + 1;
    pthread_mutex_unlock(&m);

    x = pthread_mutex_lock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }

    x = pthread_mutex_trylock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }
}

void f4() {
    int x;

    pthread_mutex_lock(&m);
    n = n + 1;
    pthread_mutex_unlock(&m);

    x = pthread_mutex_trylock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }

    x = pthread_mutex_lock(&m);
    if (x) {
        return;
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }
}

void *t_fun(void *arg) {
    f1();
    f2();
    f3();
    f4();
    return NULL;
}

int main() {
    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}
