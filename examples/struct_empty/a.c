#include <stdlib.h>
#include <pthread.h>

typedef struct {
    int n;
    pthread_mutex_t m;
} ss;

ss s1 = { 0, PTHREAD_MUTEX_INITIALIZER };

void f1(ss *s) {
    pthread_mutex_lock(&s->m);
    pthread_mutex_unlock(&s->m);
}

void *t_fun(void *arg) {
    f1(&s1);
    f1(arg);
    return NULL;
}

int main() {
    ss *s;
    s = malloc(sizeof(ss));
    s->n = 0;
    pthread_mutex_init(&s->m, NULL);

    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, s);
    pthread_create(&id2, NULL, t_fun, s);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}
