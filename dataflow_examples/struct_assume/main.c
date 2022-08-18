/* Generated by CIL v. 1.8.2 */
/* print_CIL_Input is true */

struct __pthread_internal_list {
   struct __pthread_internal_list *__prev ;
   struct __pthread_internal_list *__next ;
};
typedef struct __pthread_internal_list __pthread_list_t;
struct __pthread_mutex_s {
   int __lock ;
   unsigned int __count ;
   int __owner ;
   unsigned int __nusers ;
   int __kind ;
   short __spins ;
   short __elision ;
   __pthread_list_t __list ;
};
union __anonunion_pthread_mutex_t_335460617 {
   struct __pthread_mutex_s __data ;
   char __size[40] ;
   long __align ;
};
typedef union __anonunion_pthread_mutex_t_335460617 pthread_mutex_t;
struct __anonstruct_ss1_672045599 {
   int n ;
   pthread_mutex_t m ;
};
typedef struct __anonstruct_ss1_672045599 ss1;
struct __anonstruct_ss2_672045600 {
   int n ;
   pthread_mutex_t m ;
};
typedef struct __anonstruct_ss2_672045600 ss2;
#pragma merger("0","/tmp/cil-k0MO92ZK.i","-pthread")
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_mutex_lock)(pthread_mutex_t *__mutex ) ;
extern  __attribute__((__nothrow__)) int ( __attribute__((__nonnull__(1))) pthread_mutex_unlock)(pthread_mutex_t *__mutex ) ;
int n  =    0;
pthread_mutex_t m  =    {{0, 0U, 0, 0U, 0, (short)0, (short)0, {(struct __pthread_internal_list *)0, (struct __pthread_internal_list *)0}}};
void f2(ss1 *s ) ;
void f1(ss1 *s1 , ss2 *s2 ) 
{ 


  {
  pthread_mutex_lock(& m);
  pthread_mutex_lock(& s1->m);
  pthread_mutex_lock(& s2->m);
  n ++;
  (s1->n) ++;
  (s2->n) ++;
  f2(s1);
  pthread_mutex_unlock(& m);
  pthread_mutex_unlock(& s1->m);
  pthread_mutex_unlock(& s2->m);
  return;
}
}
void f2(ss1 *s ) 
{ 


  {
  n ++;
  (s->n) ++;
  return;
}
}
int main(void) 
{ 


  {
  return (0);
}
}
#pragma merger("0","/tmp/cil-cjugWApi.i","-pthread")