#include <pthread.h>

#define N 5

int n1 = 0;
int n2 = 0;
int n3 = 1;

pthread_mutex_t num_mutex = PTHREAD_MUTEX_INITIALIZER;

void f1() {
    int x = n3;

    pthread_mutex_lock(&num_mutex);

    n1 = n1 + x;
    n2 = n2 + x;

    pthread_mutex_unlock(&num_mutex);
}

void f2() {
    int x;

    pthread_mutex_lock(&num_mutex);

    while (n1 < 10) {
      x = n1;
      pthread_mutex_unlock(&num_mutex);

      x = x + 1;

      pthread_mutex_lock(&num_mutex);
      n1 = x;
    }

    pthread_mutex_unlock(&num_mutex);
}

void f3() {
    pthread_mutex_lock(&num_mutex);

    while (n1 < 20) {
      if (n1 > 18) {
        pthread_mutex_unlock(&num_mutex);
        return;
      }

      n1 = n1 + 1;
    }

    pthread_mutex_unlock(&num_mutex);
}

void lock() {
    pthread_mutex_lock(&num_mutex);
}

void unlock() {
    pthread_mutex_unlock(&num_mutex);
}

void f4() {
    lock();
    n1 = n1 + 1;
    unlock();
}

void inc() {
    n1 = n1 + 1;
}

void f5() {
    pthread_mutex_lock(&num_mutex);
    inc();
    pthread_mutex_unlock(&num_mutex);
}

int lock2(int n) {
    pthread_mutex_lock(&num_mutex);
    return n1 = n1 + n;
}

int unlock2(int n) {
    int n2 = n1 = n1 + n;
    pthread_mutex_unlock(&num_mutex);
    return n2;
}

int f6() {
    int n2 = lock2(1);
    n1 = n1 + 1;
    n2 = n2 + unlock2(1);
    return n2;
}

void *t_fun(void *arg) {
  f1();
  f2();
  f3();
  f4();
  f5();
  f6();
  return NULL;
}

int main() {
  pthread_t id1, id2;
  pthread_create(&id1, NULL, t_fun, NULL);
  pthread_create(&id2, NULL, t_fun, NULL);
  pthread_join(id1, NULL);
  pthread_join(id2, NULL);
}
