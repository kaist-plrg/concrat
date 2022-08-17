#include <stdlib.h>
#include <pthread.h>

typedef struct {
    int n;
    pthread_mutex_t m;
} ss;

ss *x;

void f1() {
    ss *s = malloc(sizeof(ss));
    s->n = 123;
    pthread_mutex_init(&s->m, NULL);

    x = s;

    pthread_mutex_lock(&s->m);
    s->n = 456;
    pthread_mutex_unlock(&s->m);
}

void f2() {
    ss *s = malloc(sizeof(ss));
    pthread_mutex_init(&s->m, NULL);

    x = s;

    pthread_mutex_lock(&s->m);
    s->n = 789;
    pthread_mutex_unlock(&s->m);
}

void *t_fun(void *arg) {
    f1();
    f2();
    return NULL;
}

int main() {
    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}
