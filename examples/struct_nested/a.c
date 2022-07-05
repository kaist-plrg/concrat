#include <pthread.h>

typedef struct {
    int n1;
    int n2;
} ss0;

typedef struct {
    ss0 s;
    pthread_mutex_t m;
} ss;

ss s = { { 0, 1 }, PTHREAD_MUTEX_INITIALIZER };

void f1() {
    pthread_mutex_lock(&s.m);
    s.s.n1 = s.s.n1 + 1;
    s.s.n2 = s.s.n2 + 1;
    pthread_mutex_unlock(&s.m);
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
