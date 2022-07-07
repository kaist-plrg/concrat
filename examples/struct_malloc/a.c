#include <stdlib.h>
#include <pthread.h>

typedef struct {
    int n;
    pthread_mutex_t m;
    pthread_cond_t c;
} ss;

void f1(ss *s) {
    pthread_mutex_lock(&s->m);
    s->n = s->n + 1;
    if (s->n == 1) {
        pthread_cond_wait(&s->c, &s->m);
    } else {
        pthread_cond_signal(&s->c);
    }
    pthread_mutex_unlock(&s->m);
}

void *t_fun(void *arg) {
    f1(arg);
    return NULL;
}

int main() {
    ss *s;
    pthread_t id1, id2;

    s = malloc(sizeof(ss));
    s->n = 0;
    pthread_mutex_init(&s->m, NULL);
    pthread_cond_init(&s->c, NULL);

    pthread_create(&id1, NULL, t_fun, s);
    pthread_create(&id2, NULL, t_fun, s);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);

    free(s);
}
