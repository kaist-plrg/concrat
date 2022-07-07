#include <pthread.h>

#define N 5

int n1[N];

pthread_mutex_t num_mutex[N] = {
    PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER,
    PTHREAD_MUTEX_INITIALIZER
};

void f1() {
    for (int i = 0; i < N; i++) {
        pthread_mutex_lock(&num_mutex[i]);
        n1[i] = n1[i] + 1;
        pthread_mutex_unlock(&num_mutex[i]);
    }
}

void *t_fun(void *arg) {
    f1();
    return NULL;
}

int main() {
    for (int i = 0; i < N; i++)
        n1[i] = n1[i] + 1;

    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);

    for (int i = 0; i < N; i++)
        n1[i] = n1[i] + 1;
}
