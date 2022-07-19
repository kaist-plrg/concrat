#include <pthread.h>

struct {
    int n1;
    int n2;
    int n3;
    pthread_mutex_t m1;
    pthread_cond_t cond;
} s = { 0, 0, 0, PTHREAD_MUTEX_INITIALIZER, PTHREAD_COND_INITIALIZER };

void f1() {
    struct timespec ts;
    pthread_mutex_lock(&s.m1);
    s.n1 = s.n1 + 1;
    if (s.n1 == 1) {
        clock_gettime(0, &ts);
        ts.tv_sec += (time_t) 1;
        pthread_cond_timedwait(&s.cond, &s.m1, &ts);
    } else {
        pthread_cond_signal(&s.cond);
    }
    pthread_mutex_unlock(&s.m1);
}

void f2() {
    struct timespec ts;
    pthread_mutex_lock(&s.m1);
    s.n2 = s.n2 + 1;
    if (s.n2 == 1) {
        clock_gettime(0, &ts);
        ts.tv_nsec += (long) 1;
        pthread_cond_timedwait(&s.cond, &s.m1, &ts);
    } else {
        pthread_cond_signal(&s.cond);
    }
    pthread_mutex_unlock(&s.m1);
}

void f3() {
    struct timespec ts;
    pthread_mutex_lock(&s.m1);
    s.n3 = s.n3 + 1;
    if (s.n3 == 1) {
        clock_gettime(0, &ts);
        ts.tv_sec += (time_t) 1;
        ts.tv_nsec += (long) 2;
        pthread_cond_timedwait(&s.cond, &s.m1, &ts);
    } else {
        pthread_cond_signal(&s.cond);
    }
    pthread_mutex_unlock(&s.m1);
}

void *t_fun(void *arg) {
    f1();
    f2();
    f3();
    return NULL;
}

int main() {
    pthread_t id1, id2;
    pthread_create(&id1, NULL, t_fun, NULL);
    pthread_create(&id2, NULL, t_fun, NULL);
    pthread_join(id1, NULL);
    pthread_join(id2, NULL);
}
