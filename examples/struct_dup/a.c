#include <pthread.h>

typedef struct {
    int n1;
    int n2;
    pthread_mutex_t m1;
} ss1;

typedef struct {
    int n1;
    int n3;
    pthread_mutex_t m2;
} ss2;

ss1 s1 = { 0, 1, PTHREAD_MUTEX_INITIALIZER };
ss2 s2 = { 2, 3, PTHREAD_MUTEX_INITIALIZER };

void f1() {
    int x = s1.n2 + s2.n3;

    pthread_mutex_lock(&s1.m1);
    pthread_mutex_lock(&s2.m2);

    s1.n1 = s1.n1 + x;
    s2.n1 = s2.n1 + x;

    pthread_mutex_unlock(&s1.m1);
    pthread_mutex_unlock(&s2.m2);
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
