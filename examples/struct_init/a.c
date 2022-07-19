#include <stdlib.h>
#include <pthread.h>

typedef struct {
    int n;
    pthread_mutex_t m;
} ss;

ss s1, s2;

void f1(ss *s) {
    pthread_mutex_lock(&s->m);
    s->n = s->n + 1;
    pthread_mutex_unlock(&s->m);
}

void *t_fun(void *arg) {
    f1(&s1);
    f1(&s2);
    f1(arg);
    return NULL;
}

int main() {
    ss *s3;
    pthread_t id1, id2;

    pthread_mutex_init(&s1.m, NULL);
    s1.n = 0;

    pthread_mutex_init(&s2.m, NULL);

    s3 = malloc(sizeof(ss));
    s3->n = 0;
    pthread_mutex_init(&s3->m, NULL);

    pthread_create(&id1, NULL, t_fun, s3);
    pthread_create(&id2, NULL, t_fun, s3);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}
