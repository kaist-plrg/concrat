#include <pthread.h>

int n = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    if (pthread_mutex_trylock(&m)) {
        pthread_mutex_lock(&m);
        n = n + 1;
        pthread_mutex_unlock(&m);
    } else {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }

    if (!pthread_mutex_trylock(&m)) {
        n = n + 1;
        pthread_mutex_unlock(&m);
    } else {
        pthread_mutex_lock(&m);
        n = n + 1;
        pthread_mutex_unlock(&m);
    }

    if (pthread_mutex_trylock(&m)) {
        pthread_mutex_lock(&m);
    }
    n = n + 1;
    pthread_mutex_unlock(&m);

    if (!pthread_mutex_trylock(&m)) {
        n = n + 1;
        pthread_mutex_unlock(&m);
    }
}

void *t_fun(void *arg) {
    f1();
    return NULL;
}

int main() {
    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}
