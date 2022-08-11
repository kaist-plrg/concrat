#include <pthread.h>
#include <stdlib.h>

typedef struct {
    int n;
    pthread_mutex_t m;
} ss;

void f1() {
    ss *s = malloc(sizeof(ss));
    s->n = 0;
    pthread_mutex_init(&s->m, NULL);
}

void f2(ss *s) {
    s->n = 0;
    pthread_mutex_destroy(&s->m);
    free(s);
}

void f3(ss *s) {
    pthread_mutex_lock(&s->m);
    s->n++;
    pthread_mutex_unlock(&s->m);
}

int main() {}
