#include <pthread.h>

int n = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;

typedef struct {
    int n;
    pthread_mutex_t m;
} ss1;

typedef struct {
    int n;
    pthread_mutex_t m;
} ss2;

void f1(ss1 *s1, ss2 *s2) {
    pthread_mutex_lock(&m);
    pthread_mutex_lock(&s1->m);
    pthread_mutex_lock(&s2->m);
    n++;
    s1->n++;
    s2->n++;
    f2(s1);
    pthread_mutex_unlock(&m);
    pthread_mutex_unlock(&s1->m);
    pthread_mutex_unlock(&s2->m);
}

void f2(ss1 *s) {
    n++;
    s->n++;
}

int main() {}
