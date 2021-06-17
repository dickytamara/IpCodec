
pub use pj_sys::*;



    // pj_uint16_t 	pj_ntohs (pj_uint16_t netshort)

    // pj_uint16_t 	pj_htons (pj_uint16_t hostshort)

    // pj_uint32_t 	pj_ntohl (pj_uint32_t netlong)

    // pj_uint32_t 	pj_htonl (pj_uint32_t hostlong)

    // char * 	pj_inet_ntoa (pj_in_addr inaddr)

    // int 	pj_inet_aton (const pj_str_t *cp, struct pj_in_addr *inp)

    // pj_status_t 	pj_inet_pton (int af, const pj_str_t *src, void *dst)

    // pj_status_t 	pj_inet_ntop (int af, const void *src, char *dst, int size)

    // char * 	pj_inet_ntop2 (int af, const void *src, char *dst, int size)

    // char * 	pj_sockaddr_print (const pj_sockaddr_t *addr, char *buf, int size, unsigned flags)

    // pj_in_addr 	pj_inet_addr (const pj_str_t *cp)

    // pj_in_addr 	pj_inet_addr2 (const char *cp)

    // pj_status_t 	pj_sockaddr_in_init (pj_sockaddr_in *addr, const pj_str_t *cp, pj_uint16_t port)

    // pj_status_t 	pj_sockaddr_init (int af, pj_sockaddr *addr, const pj_str_t *cp, pj_uint16_t port)

    // int 	pj_sockaddr_cmp (const pj_sockaddr_t *addr1, const pj_sockaddr_t *addr2)

    // void * 	pj_sockaddr_get_addr (const pj_sockaddr_t *addr)

    // pj_bool_t 	pj_sockaddr_has_addr (const pj_sockaddr_t *addr)

    // unsigned 	pj_sockaddr_get_addr_len (const pj_sockaddr_t *addr)

    // unsigned 	pj_sockaddr_get_len (const pj_sockaddr_t *addr)

    // void 	pj_sockaddr_copy_addr (pj_sockaddr *dst, const pj_sockaddr *src)

    // void 	pj_sockaddr_cp (pj_sockaddr_t *dst, const pj_sockaddr_t *src)

    // pj_status_t 	pj_sockaddr_synthesize (int dst_af, pj_sockaddr_t *dst, const pj_sockaddr_t *src)

    // pj_in_addr 	pj_sockaddr_in_get_addr (const pj_sockaddr_in *addr)

    // void 	pj_sockaddr_in_set_addr (pj_sockaddr_in *addr, pj_uint32_t hostaddr)

    // pj_status_t 	pj_sockaddr_in_set_str_addr (pj_sockaddr_in *addr, const pj_str_t *cp)

    // pj_status_t 	pj_sockaddr_set_str_addr (int af, pj_sockaddr *addr, const pj_str_t *cp)

    // pj_uint16_t 	pj_sockaddr_get_port (const pj_sockaddr_t *addr)

    // pj_uint16_t 	pj_sockaddr_in_get_port (const pj_sockaddr_in *addr)

    // pj_status_t 	pj_sockaddr_set_port (pj_sockaddr *addr, pj_uint16_t hostport)

    // void 	pj_sockaddr_in_set_port (pj_sockaddr_in *addr, pj_uint16_t hostport)

    // pj_status_t 	pj_sockaddr_parse (int af, unsigned options, const pj_str_t *str, pj_sockaddr *addr)

    // pj_status_t 	pj_sockaddr_parse2 (int af, unsigned options, const pj_str_t *str, pj_str_t *hostpart, pj_uint16_t *port, int *raf)

    // const pj_str_t * 	pj_gethostname (void)

    // pj_in_addr 	pj_gethostaddr (void)

    // pj_status_t 	pj_sock_socket (int family, int type, int protocol, pj_sock_t *sock)

    // pj_status_t 	pj_sock_close (pj_sock_t sockfd)

    // pj_status_t 	pj_sock_bind (pj_sock_t sockfd, const pj_sockaddr_t *my_addr, int addrlen)

    // pj_status_t 	pj_sock_bind_in (pj_sock_t sockfd, pj_uint32_t addr, pj_uint16_t port)

    // pj_status_t 	pj_sock_bind_random (pj_sock_t sockfd, const pj_sockaddr_t *addr, pj_uint16_t port_range, pj_uint16_t max_try)

    // pj_status_t 	pj_sock_listen (pj_sock_t sockfd, int backlog)

    // pj_status_t 	pj_sock_accept (pj_sock_t serverfd, pj_sock_t *newsock, pj_sockaddr_t *addr, int *addrlen)

    // pj_status_t 	pj_sock_connect (pj_sock_t sockfd, const pj_sockaddr_t *serv_addr, int addrlen)

    // pj_status_t 	pj_sock_getpeername (pj_sock_t sockfd, pj_sockaddr_t *addr, int *namelen)

    // pj_status_t 	pj_sock_getsockname (pj_sock_t sockfd, pj_sockaddr_t *addr, int *namelen)

    // pj_status_t 	pj_sock_getsockopt (pj_sock_t sockfd, pj_uint16_t level, pj_uint16_t optname, void *optval, int *optlen)

    // pj_status_t 	pj_sock_setsockopt (pj_sock_t sockfd, pj_uint16_t level, pj_uint16_t optname, const void *optval, int optlen)

    // pj_status_t 	pj_sock_setsockopt_params (pj_sock_t sockfd, const pj_sockopt_params *params)

    // pj_status_t 	pj_sock_setsockopt_sobuf (pj_sock_t sockfd, pj_uint16_t optname, pj_bool_t auto_retry, unsigned *buf_size)

    // pj_status_t 	pj_sock_recv (pj_sock_t sockfd, void *buf, pj_ssize_t *len, unsigned flags)

    // pj_status_t 	pj_sock_recvfrom (pj_sock_t sockfd, void *buf, pj_ssize_t *len, unsigned flags, pj_sockaddr_t *from, int *fromlen)

    // pj_status_t 	pj_sock_send (pj_sock_t sockfd, const void *buf, pj_ssize_t *len, unsigned flags)

    // pj_status_t 	pj_sock_sendto (pj_sock_t sockfd, const void *buf, pj_ssize_t *len, unsigned flags, const pj_sockaddr_t *to, int tolen)

    // pj_status_t 	pj_sock_shutdown (pj_sock_t sockfd, int how)

    // char * 	pj_addr_str_print (const pj_str_t *host_str, int port, char *buf, int size, unsigned flag)
