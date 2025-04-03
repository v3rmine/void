##
##   FUSE: Filesystem in Userspace
##   Version: 3.11.0
##   Copyright (C) 2001-2007  Miklos Szeredi <miklos@szeredi.hu>
##
##   Implementation of the high-level FUSE API on top of the low-level
##   API.
##
##   This program can be distributed under the terms of the GNU LGPLv2.
##   See the file COPYING.LIB
##

##  For pthread_rwlock_t

import
  config, fuse_i, fuse_lowlevel, fuse_opt, fuse_misc, fuse_kernel

const
  FUSE_NODE_SLAB* = 1

when not defined(MAP_ANONYMOUS):
  discard
when not defined(RENAME_EXCHANGE):
  const
    RENAME_EXCHANGE* = (1 shl 1)  ##  Exchange source and dest
const
  FUSE_DEFAULT_INTR_SIGNAL* = SIGUSR1
  FUSE_UNKNOWN_INO* = 0xffffffff
  OFFSET_MAX* = 0x7fffffffffffffff'i64
  NODE_TABLE_MIN_SIZE* = 8192

type
  fuse_fs* {.bycopy.} = object
    op*: fuse_operations
    m*: ptr fuse_module
    user_data*: pointer
    debug*: cint

  fusemod_so* {.bycopy.} = object
    handle*: pointer
    ctr*: cint

  lock_queue_element* {.bycopy.} = object
    next*: ptr lock_queue_element
    cond*: pthread_cond_t
    nodeid1*: fuse_ino_t
    name1*: cstring
    path1*: cstringArray
    wnode1*: ptr ptr node
    nodeid2*: fuse_ino_t
    name2*: cstring
    path2*: cstringArray
    wnode2*: ptr ptr node
    err*: cint
    first_locked* {.bitsize: 1.}: bool
    second_locked* {.bitsize: 1.}: bool
    done* {.bitsize: 1.}: bool

  node_table* {.bycopy.} = object
    array*: ptr ptr node
    use*: csize_t
    size*: csize_t
    split*: csize_t


template list_entry*(`ptr`, `type`, member: untyped): void =
  var __mptr: ptr typeof((cast[ptr `type`](0)).member) = (`ptr`)
  cast[ptr `type`]((cast[cstring](__mptr) - offsetof(`type`, member)))

type
  list_head* {.bycopy.} = object
    next*: ptr list_head
    prev*: ptr list_head

  node_slab* {.bycopy.} = object
    list*: list_head           ##  must be the first member
    freelist*: list_head
    used*: cint

  fuse* {.bycopy.} = object
    se*: ptr fuse_session
    name_table*: node_table
    id_table*: node_table
    lru_table*: list_head
    ctr*: fuse_ino_t
    generation*: cuint
    hidectr*: cuint
    lock*: pthread_mutex_t
    conf*: fuse_config
    intr_installed*: cint
    fs*: ptr fuse_fs
    lockq*: ptr lock_queue_element
    pagesize*: cint
    partial_slabs*: list_head
    full_slabs*: list_head
    prune_thread*: pthread_t

  lock* {.bycopy.} = object
    `type`*: cint
    start*: off_t
    `end`*: off_t
    pid*: pid_t
    owner*: uint64_t
    next*: ptr lock

  node* {.bycopy.} = object
    name_next*: ptr node
    id_next*: ptr node
    nodeid*: fuse_ino_t
    generation*: cuint
    refctr*: cint
    parent*: ptr node
    name*: cstring
    nlookup*: uint64_t
    open_count*: cint
    stat_updated*: timespec
    mtime*: timespec
    size*: off_t
    locks*: ptr lock
    is_hidden* {.bitsize: 1.}: cuint
    cache_valid* {.bitsize: 1.}: cuint
    treelock*: cint
    inline_name*: array[32, char]


const
  TREELOCK_WRITE* = -1
  TREELOCK_WAIT_OFFSET* = INT_MIN

type
  node_lru* {.bycopy.} = object
    node*: node
    lru*: list_head
    forget_time*: timespec

  fuse_direntry* {.bycopy.} = object
    stat*: stat
    name*: cstring
    next*: ptr fuse_direntry

  fuse_dh* {.bycopy.} = object
    lock*: pthread_mutex_t
    fuse*: ptr fuse
    req*: fuse_req_t
    contents*: cstring
    first*: ptr fuse_direntry
    last*: ptr ptr fuse_direntry
    len*: cuint
    size*: cuint
    needlen*: cuint
    filled*: cint
    fh*: uint64_t
    error*: cint
    nodeid*: fuse_ino_t

  fuse_context_i* {.bycopy.} = object
    ctx*: fuse_context
    req*: fuse_req_t


##  Defined by FUSE_REGISTER_MODULE() in lib/modules/subdir.c and iconv.c.

var fuse_module_subdir_factory*: fuse_module_factory_t

when defined(HAVE_ICONV):
  var fuse_module_iconv_factory*: fuse_module_factory_t
var fuse_context_key*: pthread_key_t

var fuse_context_lock*: pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER

var fuse_context_ref*: cint

var fuse_modules*: ptr fuse_module = nil

proc fuse_register_module*(name: cstring; factory: fuse_module_factory_t;
                          so: ptr fusemod_so): cint =
  var `mod`: ptr fuse_module
  `mod` = calloc(1, sizeof(fuse_module))
  if not `mod`:
    fuse_log(FUSE_LOG_ERR, "fuse: failed to allocate module\n")
    return -1
  `mod`.name = strdup(name)
  if not `mod`.name:
    fuse_log(FUSE_LOG_ERR, "fuse: failed to allocate module name\n")
    free(`mod`)
    return -1
  `mod`.factory = factory
  `mod`.ctr = 0
  `mod`.so = so
  if `mod`.so:
    inc(`mod`.so.ctr)
  `mod`.next = fuse_modules
  fuse_modules = `mod`
  return 0

proc fuse_unregister_module*(m: ptr fuse_module) =
  var mp: ptr ptr fuse_module
  mp = addr(fuse_modules)
  while mp[]:
    if mp[] == m:
      mp[] = (mp[]).next
      break
    mp = addr((mp[]).next)
  free(m.name)
  free(m)

proc fuse_load_so_module*(module: cstring): cint =
  var ret: cint = -1
  var tmp: cstring
  var so: ptr fusemod_so
  var factory: fuse_module_factory_t
  tmp = malloc(strlen(module) + 64)
  if not tmp:
    fuse_log(FUSE_LOG_ERR, "fuse: memory allocation failed\n")
    return -1
  sprintf(tmp, "libfusemod_%s.so", module)
  so = calloc(1, sizeof(fusemod_so))
  if not so:
    fuse_log(FUSE_LOG_ERR, "fuse: failed to allocate module so\n")
    break `out`
  so.handle = dlopen(tmp, RTLD_NOW)
  if so.handle == nil:
    fuse_log(FUSE_LOG_ERR, "fuse: dlopen(%s) failed: %s\n", tmp, dlerror())
    break out_free_so
  sprintf(tmp, "fuse_module_%s_factory", module)
  cast[ptr pointer]((addr(factory)))[] = dlsym(so.handle, tmp)
  if factory == nil:
    fuse_log(FUSE_LOG_ERR, "fuse: symbol <%s> not found in module: %s\n", tmp,
             dlerror())
    break out_dlclose
  ret = fuse_register_module(module, factory, so)
  if ret:
    break out_dlclose
  free(tmp)
  return ret
  dlclose(so.handle)
  free(so)
  break `out`

proc fuse_find_module*(module: cstring): ptr fuse_module =
  var m: ptr fuse_module
  m = fuse_modules
  while m:
    if strcmp(module, m.name) == 0:
      inc(m.ctr)
      break
    m = m.next
  return m

proc fuse_get_module*(module: cstring): ptr fuse_module =
  var m: ptr fuse_module
  pthread_mutex_lock(addr(fuse_context_lock))
  m = fuse_find_module(module)
  if not m:
    var err: cint = fuse_load_so_module(module)
    if not err:
      m = fuse_find_module(module)
  pthread_mutex_unlock(addr(fuse_context_lock))
  return m

proc fuse_put_module*(m: ptr fuse_module) =
  pthread_mutex_lock(addr(fuse_context_lock))
  if m.so:
    assert(m.ctr > 0)
  if m.ctr > 0:
    dec(m.ctr)
  if not m.ctr and m.so:
    var so: ptr fusemod_so = m.so
    assert(so.ctr > 0)
    dec(so.ctr)
    if not so.ctr:
      var mp: ptr ptr fuse_module
      mp = addr(fuse_modules)
      while mp[]:
        if (mp[]).so == so:
          fuse_unregister_module(mp[])
        else:
          mp = addr((mp[]).next)
      dlclose(so.handle)
      free(so)
  elif not m.ctr:
    fuse_unregister_module(m)
  pthread_mutex_unlock(addr(fuse_context_lock))

proc init_list_head*(list: ptr list_head) =
  list.next = list
  list.prev = list

proc list_empty*(head: ptr list_head): cint =
  return head.next == head

proc list_add*(new: ptr list_head; prev: ptr list_head; next: ptr list_head) =
  next.prev = new
  new.next = next
  new.prev = prev
  prev.next = new

proc list_add_head*(new: ptr list_head; head: ptr list_head) {.inline.} =
  list_add(new, head, head.next)

proc list_add_tail*(new: ptr list_head; head: ptr list_head) {.inline.} =
  list_add(new, head.prev, head)

proc list_del*(entry: ptr list_head) {.inline.} =
  var prev: ptr list_head = entry.prev
  var next: ptr list_head = entry.next
  next.prev = prev
  prev.next = next

proc lru_enabled*(f: ptr fuse): cint {.inline.} =
  return f.conf.remember > 0

proc node_lru*(node: ptr node): ptr node_lru =
  return cast[ptr node_lru](node)

proc get_node_size*(f: ptr fuse): csize_t =
  if lru_enabled(f):
    return sizeof(node_lru)
  else:
    return sizeof(node)

when defined(FUSE_NODE_SLAB):
  proc list_to_slab*(head: ptr list_head): ptr node_slab =
    return cast[ptr node_slab](head)

  proc node_to_slab*(f: ptr fuse; node: ptr node): ptr node_slab =
    return cast[ptr node_slab](((cast[uintptr_t](node)) and
        not (cast[uintptr_t](f.pagesize) - 1)))

  proc alloc_slab*(f: ptr fuse): cint =
    var mem: pointer
    var slab: ptr node_slab
    var start: cstring
    var num: csize_t
    var i: csize_t
    var node_size: csize_t = get_node_size(f)
    mem = mmap(nil, f.pagesize, PROT_READ or PROT_WRITE, MAP_PRIVATE or MAP_ANONYMOUS,
             -1, 0)
    if mem == MAP_FAILED:
      return -1
    slab = mem
    init_list_head(addr(slab.freelist))
    slab.used = 0
    num = (f.pagesize - sizeof(node_slab)) div node_size
    start = cast[cstring](mem) + f.pagesize - num * node_size
    i = 0
    while i < num:
      var n: ptr list_head
      n = cast[ptr list_head]((start + i * node_size))
      list_add_tail(n, addr(slab.freelist))
      inc(i)
    list_add_tail(addr(slab.list), addr(f.partial_slabs))
    return 0

  proc alloc_node*(f: ptr fuse): ptr node =
    var slab: ptr node_slab
    var node: ptr list_head
    if list_empty(addr(f.partial_slabs)):
      var res: cint = alloc_slab(f)
      if res != 0:
        return nil
    slab = list_to_slab(f.partial_slabs.next)
    inc(slab.used)
    node = slab.freelist.next
    list_del(node)
    if list_empty(addr(slab.freelist)):
      list_del(addr(slab.list))
      list_add_tail(addr(slab.list), addr(f.full_slabs))
    memset(node, 0, sizeof(node))
    return cast[ptr node](node)

  proc free_slab*(f: ptr fuse; slab: ptr node_slab) =
    var res: cint
    list_del(addr(slab.list))
    res = munmap(slab, f.pagesize)
    if res == -1:
      fuse_log(FUSE_LOG_WARNING, "fuse warning: munmap(%p) failed\n", slab)

  proc free_node_mem*(f: ptr fuse; node: ptr node) =
    var slab: ptr node_slab = node_to_slab(f, node)
    var n: ptr list_head = cast[ptr list_head](node)
    dec(slab.used)
    if slab.used:
      if list_empty(addr(slab.freelist)):
        list_del(addr(slab.list))
        list_add_tail(addr(slab.list), addr(f.partial_slabs))
      list_add_head(n, addr(slab.freelist))
    else:
      free_slab(f, slab)

else:
  proc alloc_node*(f: ptr fuse): ptr node =
    return cast[ptr node](calloc(1, get_node_size(f)))

  proc free_node_mem*(f: ptr fuse; node: ptr node) =
    cast[nil](f)
    free(node)

proc id_hash*(f: ptr fuse; ino: fuse_ino_t): csize_t =
  var hash: uint64_t = (cast[uint32_t](ino * 2654435761U'i64)) mod f.id_table.size
  var oldhash: uint64_t = hash mod (f.id_table.size div 2)
  if oldhash >= f.id_table.split:
    return oldhash
  else:
    return hash

proc get_node_nocheck*(f: ptr fuse; nodeid: fuse_ino_t): ptr node =
  var hash: csize_t = id_hash(f, nodeid)
  var node: ptr node
  node = f.id_table.array[hash]
  while node != nil:
    if node.nodeid == nodeid:
      return node
    node = node.id_next
  return nil

proc get_node*(f: ptr fuse; nodeid: fuse_ino_t): ptr node =
  var node: ptr node = get_node_nocheck(f, nodeid)
  if not node:
    fuse_log(FUSE_LOG_ERR, "fuse internal error: node %llu not found\n",
             cast[culonglong](nodeid))
    abort()
  return node

proc curr_time*(now: ptr timespec)
proc diff_timespec*(t1: ptr timespec; t2: ptr timespec): cdouble
proc remove_node_lru*(node: ptr node) =
  var lnode: ptr node_lru = node_lru(node)
  list_del(addr(lnode.lru))
  init_list_head(addr(lnode.lru))

proc set_forget_time*(f: ptr fuse; node: ptr node) =
  var lnode: ptr node_lru = node_lru(node)
  list_del(addr(lnode.lru))
  list_add_tail(addr(lnode.lru), addr(f.lru_table))
  curr_time(addr(lnode.forget_time))

proc free_node*(f: ptr fuse; node: ptr node) =
  if node.name != node.inline_name:
    free(node.name)
  free_node_mem(f, node)

proc node_table_reduce*(t: ptr node_table) =
  var newsize: csize_t = t.size div 2
  var newarray: pointer
  if newsize < NODE_TABLE_MIN_SIZE:
    return
  newarray = realloc(t.array, sizeof(cast[ptr node](newsize[])))
  if newarray != nil:
    t.array = newarray
  t.size = newsize
  t.split = t.size div 2

proc remerge_id*(f: ptr fuse) =
  var t: ptr node_table = addr(f.id_table)
  var iter: cint
  if t.split == 0:
    node_table_reduce(t)
  iter = 8
  while t.split > 0 and iter:
    var upper: ptr ptr node
    dec(t.split)
    upper = addr(t.array[t.split + t.size div 2])
    if upper[]:
      var nodep: ptr ptr node
      nodep = addr(t.array[t.split])
      while nodep[]:
        ## ignored statement
        nodep = addr((nodep[]).id_next)
      nodep[] = upper[]
      upper[] = nil
      break
    dec(iter)

proc unhash_id*(f: ptr fuse; node: ptr node) =
  var nodep: ptr ptr node = addr(f.id_table.array[id_hash(f, node.nodeid)])
  while nodep[] != nil:
    if nodep[] == node:
      nodep[] = node.id_next
      dec(f.id_table.use)
      if f.id_table.use < f.id_table.size div 4:
        remerge_id(f)
      return
    nodep = addr((nodep[]).id_next)

proc node_table_resize*(t: ptr node_table): cint =
  var newsize: csize_t = t.size * 2
  var newarray: pointer
  newarray = realloc(t.array, sizeof(cast[ptr node](newsize[])))
  if newarray == nil:
    return -1
  t.array = newarray
  memset(t.array + t.size, 0, t.size * sizeof(ptr node))
  t.size = newsize
  t.split = 0
  return 0

proc rehash_id*(f: ptr fuse) =
  var t: ptr node_table = addr(f.id_table)
  var nodep: ptr ptr node
  var next: ptr ptr node
  var hash: csize_t
  if t.split == t.size div 2:
    return
  hash = t.split
  inc(t.split)
  nodep = addr(t.array[hash])
  while nodep[] != nil:
    var node: ptr node = nodep[]
    var newhash: csize_t = id_hash(f, node.nodeid)
    if newhash != hash:
      next = nodep
      nodep[] = node.id_next
      node.id_next = t.array[newhash]
      t.array[newhash] = node
    else:
      next = addr(node.id_next)
    nodep = next
  if t.split == t.size div 2:
    node_table_resize(t)

proc hash_id*(f: ptr fuse; node: ptr node) =
  var hash: csize_t = id_hash(f, node.nodeid)
  node.id_next = f.id_table.array[hash]
  f.id_table.array[hash] = node
  inc(f.id_table.use)
  if f.id_table.use >= f.id_table.size div 2:
    rehash_id(f)

proc name_hash*(f: ptr fuse; parent: fuse_ino_t; name: cstring): csize_t =
  var hash: uint64_t = parent
  var oldhash: uint64_t
  while name[]:
    hash = hash * 31 + cast[cuchar](name[])
    inc(name)
  hash = hash mod f.name_table.size
  oldhash = hash mod (f.name_table.size div 2)
  if oldhash >= f.name_table.split:
    return oldhash
  else:
    return hash

proc unref_node*(f: ptr fuse; node: ptr node)
proc remerge_name*(f: ptr fuse) =
  var t: ptr node_table = addr(f.name_table)
  var iter: cint
  if t.split == 0:
    node_table_reduce(t)
  iter = 8
  while t.split > 0 and iter:
    var upper: ptr ptr node
    dec(t.split)
    upper = addr(t.array[t.split + t.size div 2])
    if upper[]:
      var nodep: ptr ptr node
      nodep = addr(t.array[t.split])
      while nodep[]:
        ## ignored statement
        nodep = addr((nodep[]).name_next)
      nodep[] = upper[]
      upper[] = nil
      break
    dec(iter)

proc unhash_name*(f: ptr fuse; node: ptr node) =
  if node.name:
    var hash: csize_t = name_hash(f, node.parent.nodeid, node.name)
    var nodep: ptr ptr node = addr(f.name_table.array[hash])
    while nodep[] != nil:
      if nodep[] == node:
        nodep[] = node.name_next
        node.name_next = nil
        unref_node(f, node.parent)
        if node.name != node.inline_name:
          free(node.name)
        node.name = nil
        node.parent = nil
        dec(f.name_table.use)
        if f.name_table.use < f.name_table.size div 4:
          remerge_name(f)
        return
      nodep = addr((nodep[]).name_next)
    fuse_log(FUSE_LOG_ERR, "fuse internal error: unable to unhash node: %llu\n",
             cast[culonglong](node.nodeid))
    abort()

proc rehash_name*(f: ptr fuse) =
  var t: ptr node_table = addr(f.name_table)
  var nodep: ptr ptr node
  var next: ptr ptr node
  var hash: csize_t
  if t.split == t.size div 2:
    return
  hash = t.split
  inc(t.split)
  nodep = addr(t.array[hash])
  while nodep[] != nil:
    var node: ptr node = nodep[]
    var newhash: csize_t = name_hash(f, node.parent.nodeid, node.name)
    if newhash != hash:
      next = nodep
      nodep[] = node.name_next
      node.name_next = t.array[newhash]
      t.array[newhash] = node
    else:
      next = addr(node.name_next)
    nodep = next
  if t.split == t.size div 2:
    node_table_resize(t)

proc hash_name*(f: ptr fuse; node: ptr node; parentid: fuse_ino_t; name: cstring): cint =
  var hash: csize_t = name_hash(f, parentid, name)
  var parent: ptr node = get_node(f, parentid)
  if strlen(name) < sizeof((node.inline_name)):
    strcpy(node.inline_name, name)
    node.name = node.inline_name
  else:
    node.name = strdup(name)
    if node.name == nil:
      return -1
  inc(parent.refctr)
  node.parent = parent
  node.name_next = f.name_table.array[hash]
  f.name_table.array[hash] = node
  inc(f.name_table.use)
  if f.name_table.use >= f.name_table.size div 2:
    rehash_name(f)
  return 0

proc delete_node*(f: ptr fuse; node: ptr node) =
  if f.conf.debug:
    fuse_log(FUSE_LOG_DEBUG, "DELETE: %llu\n", cast[culonglong](node.nodeid))
  assert(node.treelock == 0)
  unhash_name(f, node)
  if lru_enabled(f):
    remove_node_lru(node)
  unhash_id(f, node)
  free_node(f, node)

proc unref_node*(f: ptr fuse; node: ptr node) =
  assert(node.refctr > 0)
  dec(node.refctr)
  if not node.refctr:
    delete_node(f, node)

proc next_id*(f: ptr fuse): fuse_ino_t =
  while true:
    f.ctr = (f.ctr + 1) and 0xffffffff
    if not f.ctr:
      inc(f.generation)
    if not (f.ctr == 0 or f.ctr == FUSE_UNKNOWN_INO or
        get_node_nocheck(f, f.ctr) != nil):
      break
  return f.ctr

proc lookup_node*(f: ptr fuse; parent: fuse_ino_t; name: cstring): ptr node =
  var hash: csize_t = name_hash(f, parent, name)
  var node: ptr node
  node = f.name_table.array[hash]
  while node != nil:
    if node.parent.nodeid == parent and strcmp(node.name, name) == 0:
      return node
    node = node.name_next
  return nil

proc inc_nlookup*(node: ptr node) =
  if not node.nlookup:
    inc(node.refctr)
  inc(node.nlookup)

proc find_node*(f: ptr fuse; parent: fuse_ino_t; name: cstring): ptr node =
  var node: ptr node
  pthread_mutex_lock(addr(f.lock))
  if not name:
    node = get_node(f, parent)
  else:
    node = lookup_node(f, parent, name)
  if node == nil:
    node = alloc_node(f)
    if node == nil:
      break out_err
    node.nodeid = next_id(f)
    node.generation = f.generation
    if f.conf.remember:
      inc_nlookup(node)
    if hash_name(f, node, parent, name) == -1:
      free_node(f, node)
      node = nil
      break out_err
    hash_id(f, node)
    if lru_enabled(f):
      var lnode: ptr node_lru = node_lru(node)
      init_list_head(addr(lnode.lru))
  elif lru_enabled(f) and node.nlookup == 1:
    remove_node_lru(node)
  inc_nlookup(node)
  pthread_mutex_unlock(addr(f.lock))
  return node

proc lookup_path_in_cache*(f: ptr fuse; path: cstring; inop: ptr fuse_ino_t): cint =
  var tmp: cstring = strdup(path)
  if not tmp:
    return -ENOMEM
  pthread_mutex_lock(addr(f.lock))
  var ino: fuse_ino_t = FUSE_ROOT_ID
  var err: cint = 0
  var save_ptr: cstring
  var path_element: cstring = strtok_r(tmp, "/", addr(save_ptr))
  while path_element != nil:
    var node: ptr node = lookup_node(f, ino, path_element)
    if node == nil:
      err = -ENOENT
      break
    ino = node.nodeid
    path_element = strtok_r(nil, "/", addr(save_ptr))
  pthread_mutex_unlock(addr(f.lock))
  free(tmp)
  if not err:
    inop[] = ino
  return err

proc add_name*(buf: cstringArray; bufsize: ptr cuint; s: cstring; name: cstring): cstring =
  var len: csize_t = strlen(name)
  if s - len <= buf[]:
    var pathlen: cuint = bufsize[] - (s - buf[])
    var newbufsize: cuint = bufsize[]
    var newbuf: cstring
    while newbufsize < pathlen + len + 1:
      if newbufsize >= 0x80000000:
        newbufsize = 0xffffffff
      else:
        newbufsize = newbufsize * 2
    newbuf = realloc(buf[], newbufsize)
    if newbuf == nil:
      return nil
    buf[] = newbuf
    s = newbuf + newbufsize - pathlen
    memmove(s, newbuf + bufsize[] - pathlen, pathlen)
    bufsize[] = newbufsize
  dec(s, len)
  memcpy(s, name, len)
  dec(s)
  s[] = '/'
  return s

proc unlock_path*(f: ptr fuse; nodeid: fuse_ino_t; wnode: ptr node; `end`: ptr node) =
  var node: ptr node
  if wnode:
    assert(wnode.treelock == TREELOCK_WRITE)
    wnode.treelock = 0
  node = get_node(f, nodeid)
  while node != `end` and node.nodeid != FUSE_ROOT_ID:
    assert(node.treelock != 0)
    assert(node.treelock != TREELOCK_WAIT_OFFSET)
    assert(node.treelock != TREELOCK_WRITE)
    dec(node.treelock)
    if node.treelock == TREELOCK_WAIT_OFFSET:
      node.treelock = 0
    node = node.parent

proc try_get_path*(f: ptr fuse; nodeid: fuse_ino_t; name: cstring; path: cstringArray;
                  wnodep: ptr ptr node; need_lock: bool): cint =
  var bufsize: cuint = 256
  var buf: cstring
  var s: cstring
  var node: ptr node
  var wnode: ptr node = nil
  var err: cint
  path[] = nil
  err = -ENOMEM
  buf = malloc(bufsize)
  if buf == nil:
    break out_err
  s = buf + bufsize - 1
  s[] = '\x00'
  if name != nil:
    s = add_name(addr(buf), addr(bufsize), s, name)
    err = -ENOMEM
    if s == nil:
      break out_free
  if wnodep:
    assert(need_lock)
    wnode = lookup_node(f, nodeid, name)
    if wnode:
      if wnode.treelock != 0:
        if wnode.treelock > 0:
          inc(wnode.treelock, TREELOCK_WAIT_OFFSET)
        err = -EAGAIN
        break out_free
      wnode.treelock = TREELOCK_WRITE
  node = get_node(f, nodeid)
  while node.nodeid != FUSE_ROOT_ID:
    err = -ESTALE
    if node.name == nil or node.parent == nil:
      break out_unlock
    err = -ENOMEM
    s = add_name(addr(buf), addr(bufsize), s, node.name)
    if s == nil:
      break out_unlock
    if need_lock:
      err = -EAGAIN
      if node.treelock < 0:
        break out_unlock
      inc(node.treelock)
    node = node.parent
  if s[0]:
    memmove(buf, s, bufsize - (s - buf))
  else:
    strcpy(buf, "/")
  path[] = buf
  if wnodep:
    wnodep[] = wnode
  return 0
  if need_lock:
    unlock_path(f, nodeid, wnode, node)
  free(buf)
  return err

proc queue_element_unlock*(f: ptr fuse; qe: ptr lock_queue_element) =
  var wnode: ptr node
  if qe.first_locked:
    wnode = if qe.wnode1: qe.wnode1[] else: nil
    unlock_path(f, qe.nodeid1, wnode, nil)
    qe.first_locked = false
  if qe.second_locked:
    wnode = if qe.wnode2: qe.wnode2[] else: nil
    unlock_path(f, qe.nodeid2, wnode, nil)
    qe.second_locked = false

proc queue_element_wakeup*(f: ptr fuse; qe: ptr lock_queue_element) =
  var err: cint
  var first: bool = (qe == f.lockq)
  if not qe.path1:
    ##  Just waiting for it to be unlocked
    if get_node(f, qe.nodeid1).treelock == 0:
      pthread_cond_signal(addr(qe.cond))
    return
  if not qe.first_locked:
    err = try_get_path(f, qe.nodeid1, qe.name1, qe.path1, qe.wnode1, true)
    if not err:
      qe.first_locked = true
    elif err != -EAGAIN:
      break err_unlock
  if not qe.second_locked and qe.path2:
    err = try_get_path(f, qe.nodeid2, qe.name2, qe.path2, qe.wnode2, true)
    if not err:
      qe.second_locked = true
    elif err != -EAGAIN:
      break err_unlock
  if qe.first_locked and (qe.second_locked or not qe.path2):
    err = 0
    break done
  if not first:
    queue_element_unlock(f, qe)
  return
  queue_element_unlock(f, qe)
  qe.err = err
  qe.done = true
  pthread_cond_signal(addr(qe.cond))

proc wake_up_queued*(f: ptr fuse) =
  var qe: ptr lock_queue_element
  qe = f.lockq
  while qe != nil:
    queue_element_wakeup(f, qe)
    qe = qe.next

proc debug_path*(f: ptr fuse; msg: cstring; nodeid: fuse_ino_t; name: cstring; wr: bool) =
  if f.conf.debug:
    var wnode: ptr node = nil
    if wr:
      wnode = lookup_node(f, nodeid, name)
    if wnode:
      fuse_log(FUSE_LOG_DEBUG, "%s %llu (w)\n", msg, cast[culonglong](wnode.nodeid))
    else:
      fuse_log(FUSE_LOG_DEBUG, "%s %llu\n", msg, cast[culonglong](nodeid))

proc queue_path*(f: ptr fuse; qe: ptr lock_queue_element) =
  var qp: ptr ptr lock_queue_element
  qe.done = false
  qe.first_locked = false
  qe.second_locked = false
  pthread_cond_init(addr(qe.cond), nil)
  qe.next = nil
  qp = addr(f.lockq)
  while qp[] != nil:
    ## ignored statement
    qp = addr((qp[]).next)
  qp[] = qe

proc dequeue_path*(f: ptr fuse; qe: ptr lock_queue_element) =
  var qp: ptr ptr lock_queue_element
  pthread_cond_destroy(addr(qe.cond))
  qp = addr(f.lockq)
  while qp[] != qe:
    ## ignored statement
    qp = addr((qp[]).next)
  qp[] = qe.next

proc wait_path*(f: ptr fuse; qe: ptr lock_queue_element): cint =
  queue_path(f, qe)
  while true:
    pthread_cond_wait(addr(qe.cond), addr(f.lock))
    if not not qe.done:
      break
  dequeue_path(f, qe)
  return qe.err

proc get_path_common*(f: ptr fuse; nodeid: fuse_ino_t; name: cstring;
                     path: cstringArray; wnode: ptr ptr node): cint =
  var err: cint
  pthread_mutex_lock(addr(f.lock))
  err = try_get_path(f, nodeid, name, path, wnode, true)
  if err == -EAGAIN:
    var qe: lock_queue_element = [nodeid1: nodeid, name1: name, path1: path, wnode1: wnode]
    debug_path(f, "QUEUE PATH", nodeid, name, not not wnode)
    err = wait_path(f, addr(qe))
    debug_path(f, "DEQUEUE PATH", nodeid, name, not not wnode)
  pthread_mutex_unlock(addr(f.lock))
  return err

proc get_path*(f: ptr fuse; nodeid: fuse_ino_t; path: cstringArray): cint =
  return get_path_common(f, nodeid, nil, path, nil)

proc get_path_nullok*(f: ptr fuse; nodeid: fuse_ino_t; path: cstringArray): cint =
  var err: cint = 0
  if f.conf.nullpath_ok:
    path[] = nil
  else:
    err = get_path_common(f, nodeid, nil, path, nil)
    if err == -ESTALE:
      err = 0
  return err

proc get_path_name*(f: ptr fuse; nodeid: fuse_ino_t; name: cstring; path: cstringArray): cint =
  return get_path_common(f, nodeid, name, path, nil)

proc get_path_wrlock*(f: ptr fuse; nodeid: fuse_ino_t; name: cstring;
                     path: cstringArray; wnode: ptr ptr node): cint =
  return get_path_common(f, nodeid, name, path, wnode)

when defined(__FreeBSD__):
  discard
proc check_dir_loop*(f: ptr fuse; nodeid1: fuse_ino_t; name1: cstring;
                    nodeid2: fuse_ino_t; name2: cstring): cint =
  var
    node: ptr node
    node1: ptr node
    node2: ptr node
  var
    id1: fuse_ino_t
    id2: fuse_ino_t
  node1 = lookup_node(f, nodeid1, name1)
  id1 = if node1: node1.nodeid else: nodeid1
  node2 = lookup_node(f, nodeid2, name2)
  id2 = if node2: node2.nodeid else: nodeid2
  node = get_node(f, id2)
  while node.nodeid != FUSE_ROOT_ID:
    if node.name == nil or node.parent == nil:
      break
    if node.nodeid != id2 and node.nodeid == id1:
      return -EINVAL
    node = node.parent
  if node2:
    node = get_node(f, id1)
    while node.nodeid != FUSE_ROOT_ID:
      if node.name == nil or node.parent == nil:
        break
      if node.nodeid != id1 and node.nodeid == id2:
        return -ENOTEMPTY
      node = node.parent
  return 0

proc try_get_path2*(f: ptr fuse; nodeid1: fuse_ino_t; name1: cstring;
                   nodeid2: fuse_ino_t; name2: cstring; path1: cstringArray;
                   path2: cstringArray; wnode1: ptr ptr node; wnode2: ptr ptr node): cint =
  var err: cint
  ##  FIXME: locking two paths needs deadlock checking
  err = try_get_path(f, nodeid1, name1, path1, wnode1, true)
  if not err:
    err = try_get_path(f, nodeid2, name2, path2, wnode2, true)
    if err:
      var wn1: ptr node = if wnode1: wnode1[] else: nil
      unlock_path(f, nodeid1, wn1, nil)
      free(path1[])
  return err

proc get_path2*(f: ptr fuse; nodeid1: fuse_ino_t; name1: cstring; nodeid2: fuse_ino_t;
               name2: cstring; path1: cstringArray; path2: cstringArray;
               wnode1: ptr ptr node; wnode2: ptr ptr node): cint =
  var err: cint
  pthread_mutex_lock(addr(f.lock))
  if name1:
    ##  called during rename; perform dir loop check
    err = check_dir_loop(f, nodeid1, name1, nodeid2, name2)
    if err:
      break out_unlock
  err = try_get_path2(f, nodeid1, name1, nodeid2, name2, path1, path2, wnode1, wnode2)
  if err == -EAGAIN:
    var qe: lock_queue_element = [nodeid1: nodeid1, name1: name1, path1: path1,
                              wnode1: wnode1, nodeid2: nodeid2, name2: name2,
                              path2: path2, wnode2: wnode2]
    debug_path(f, "QUEUE PATH1", nodeid1, name1, not not wnode1)
    debug_path(f, "      PATH2", nodeid2, name2, not not wnode2)
    err = wait_path(f, addr(qe))
    debug_path(f, "DEQUEUE PATH1", nodeid1, name1, not not wnode1)
    debug_path(f, "        PATH2", nodeid2, name2, not not wnode2)
  ## !!!Ignored construct:  # defined ( ) [NewLine] out_unlock : # [NewLine] pthread_mutex_unlock ( & f -> lock ) ;
  ## Error: did not expect }!!!
  return err

proc free_path_wrlock*(f: ptr fuse; nodeid: fuse_ino_t; wnode: ptr node; path: cstring) =
  pthread_mutex_lock(addr(f.lock))
  unlock_path(f, nodeid, wnode, nil)
  if f.lockq:
    wake_up_queued(f)
  pthread_mutex_unlock(addr(f.lock))
  free(path)

proc free_path*(f: ptr fuse; nodeid: fuse_ino_t; path: cstring) =
  if path:
    free_path_wrlock(f, nodeid, nil, path)

proc free_path2*(f: ptr fuse; nodeid1: fuse_ino_t; nodeid2: fuse_ino_t;
                wnode1: ptr node; wnode2: ptr node; path1: cstring; path2: cstring) =
  pthread_mutex_lock(addr(f.lock))
  unlock_path(f, nodeid1, wnode1, nil)
  unlock_path(f, nodeid2, wnode2, nil)
  wake_up_queued(f)
  pthread_mutex_unlock(addr(f.lock))
  free(path1)
  free(path2)

proc forget_node*(f: ptr fuse; nodeid: fuse_ino_t; nlookup: uint64_t) =
  var node: ptr node
  if nodeid == FUSE_ROOT_ID:
    return
  pthread_mutex_lock(addr(f.lock))
  node = get_node(f, nodeid)
  ##
  ##  Node may still be locked due to interrupt idiocy in open,
  ##  create and opendir
  ##
  while node.nlookup == nlookup and node.treelock:
    var qe: lock_queue_element = [nodeid1: nodeid]
    debug_path(f, "QUEUE PATH (forget)", nodeid, nil, false)
    queue_path(f, addr(qe))
    while true:
      pthread_cond_wait(addr(qe.cond), addr(f.lock))
      if not (node.nlookup == nlookup and node.treelock):
        break
    dequeue_path(f, addr(qe))
    debug_path(f, "DEQUEUE_PATH (forget)", nodeid, nil, false)
  assert(node.nlookup >= nlookup)
  dec(node.nlookup, nlookup)
  if not node.nlookup:
    unref_node(f, node)
  elif lru_enabled(f) and node.nlookup == 1:
    set_forget_time(f, node)
  pthread_mutex_unlock(addr(f.lock))

proc unlink_node*(f: ptr fuse; node: ptr node) =
  if f.conf.remember:
    assert(node.nlookup > 1)
    dec(node.nlookup)
  unhash_name(f, node)

proc remove_node*(f: ptr fuse; dir: fuse_ino_t; name: cstring) =
  var node: ptr node
  pthread_mutex_lock(addr(f.lock))
  node = lookup_node(f, dir, name)
  if node != nil:
    unlink_node(f, node)
  pthread_mutex_unlock(addr(f.lock))

proc rename_node*(f: ptr fuse; olddir: fuse_ino_t; oldname: cstring; newdir: fuse_ino_t;
                 newname: cstring; hide: cint): cint =
  var node: ptr node
  var newnode: ptr node
  var err: cint = 0
  pthread_mutex_lock(addr(f.lock))
  node = lookup_node(f, olddir, oldname)
  newnode = lookup_node(f, newdir, newname)
  if node == nil:
    break `out`
  if newnode != nil:
    if hide:
      fuse_log(FUSE_LOG_ERR, "fuse: hidden file got created during hiding\n")
      err = -EBUSY
      break `out`
    unlink_node(f, newnode)
  unhash_name(f, node)
  if hash_name(f, node, newdir, newname) == -1:
    err = -ENOMEM
    break `out`
  if hide:
    node.is_hidden = 1
  pthread_mutex_unlock(addr(f.lock))
  return err

proc exchange_node*(f: ptr fuse; olddir: fuse_ino_t; oldname: cstring;
                   newdir: fuse_ino_t; newname: cstring): cint =
  var oldnode: ptr node
  var newnode: ptr node
  var err: cint
  pthread_mutex_lock(addr(f.lock))
  oldnode = lookup_node(f, olddir, oldname)
  newnode = lookup_node(f, newdir, newname)
  if oldnode:
    unhash_name(f, oldnode)
  if newnode:
    unhash_name(f, newnode)
  err = -ENOMEM
  if oldnode:
    if hash_name(f, oldnode, newdir, newname) == -1:
      break `out`
  if newnode:
    if hash_name(f, newnode, olddir, oldname) == -1:
      break `out`
  err = 0
  pthread_mutex_unlock(addr(f.lock))
  return err

proc set_stat*(f: ptr fuse; nodeid: fuse_ino_t; stbuf: ptr stat) =
  if not f.conf.use_ino:
    stbuf.st_ino = nodeid
  if f.conf.set_mode:
    stbuf.st_mode = (stbuf.st_mode and S_IFMT) or (0o777 and not f.conf.umask)
  if f.conf.set_uid:
    stbuf.st_uid = f.conf.uid
  if f.conf.set_gid:
    stbuf.st_gid = f.conf.gid

proc req_fuse*(req: fuse_req_t): ptr fuse =
  return cast[ptr fuse](fuse_req_userdata(req))

proc fuse_intr_sighandler*(sig: cint) =
  cast[nil](sig)
  ##  Nothing to do

type
  fuse_intr_data* {.bycopy.} = object
    id*: pthread_t
    cond*: pthread_cond_t
    finished*: cint


proc fuse_interrupt*(req: fuse_req_t; d_: pointer) =
  var d: ptr fuse_intr_data = d_
  var f: ptr fuse = req_fuse(req)
  if d.id == pthread_self():
    return
  pthread_mutex_lock(addr(f.lock))
  while not d.finished:
    var now: timeval
    var timeout: timespec
    pthread_kill(d.id, f.conf.intr_signal)
    gettimeofday(addr(now), nil)
    timeout.tv_sec = now.tv_sec + 1
    timeout.tv_nsec = now.tv_usec * 1000
    pthread_cond_timedwait(addr(d.cond), addr(f.lock), addr(timeout))
  pthread_mutex_unlock(addr(f.lock))

proc fuse_do_finish_interrupt*(f: ptr fuse; req: fuse_req_t; d: ptr fuse_intr_data) =
  pthread_mutex_lock(addr(f.lock))
  d.finished = 1
  pthread_cond_broadcast(addr(d.cond))
  pthread_mutex_unlock(addr(f.lock))
  fuse_req_interrupt_func(req, nil, nil)
  pthread_cond_destroy(addr(d.cond))

proc fuse_do_prepare_interrupt*(req: fuse_req_t; d: ptr fuse_intr_data) =
  d.id = pthread_self()
  pthread_cond_init(addr(d.cond), nil)
  d.finished = 0
  fuse_req_interrupt_func(req, fuse_interrupt, d)

proc fuse_finish_interrupt*(f: ptr fuse; req: fuse_req_t; d: ptr fuse_intr_data) {.inline.} =
  if f.conf.intr:
    fuse_do_finish_interrupt(f, req, d)

proc fuse_prepare_interrupt*(f: ptr fuse; req: fuse_req_t; d: ptr fuse_intr_data) {.
    inline.} =
  if f.conf.intr:
    fuse_do_prepare_interrupt(req, d)

proc file_info_string*(fi: ptr fuse_file_info; buf: cstring; len: csize_t): cstring =
  if fi == nil:
    return "NULL"
  snprintf(buf, len, "%llu", cast[culonglong](fi.fh))
  return buf

proc fuse_fs_getattr*(fs: ptr fuse_fs; path: cstring; buf: ptr stat;
                     fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.getattr:
    if fs.debug:
      var buf: array[10, char]
      fuse_log(FUSE_LOG_DEBUG, "getattr[%s] %s\n",
               file_info_string(fi, buf, sizeof((buf))), path)
    return fs.op.getattr(path, buf, fi)
  else:
    return -ENOSYS

proc fuse_fs_rename*(fs: ptr fuse_fs; oldpath: cstring; newpath: cstring; flags: cuint): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.rename:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "rename %s %s 0x%x\n", oldpath, newpath, flags)
    return fs.op.rename(oldpath, newpath, flags)
  else:
    return -ENOSYS

proc fuse_fs_unlink*(fs: ptr fuse_fs; path: cstring): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.unlink:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "unlink %s\n", path)
    return fs.op.unlink(path)
  else:
    return -ENOSYS

proc fuse_fs_rmdir*(fs: ptr fuse_fs; path: cstring): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.rmdir:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "rmdir %s\n", path)
    return fs.op.rmdir(path)
  else:
    return -ENOSYS

proc fuse_fs_symlink*(fs: ptr fuse_fs; linkname: cstring; path: cstring): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.symlink:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "symlink %s %s\n", linkname, path)
    return fs.op.symlink(linkname, path)
  else:
    return -ENOSYS

proc fuse_fs_link*(fs: ptr fuse_fs; oldpath: cstring; newpath: cstring): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.link:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "link %s %s\n", oldpath, newpath)
    return fs.op.link(oldpath, newpath)
  else:
    return -ENOSYS

proc fuse_fs_release*(fs: ptr fuse_fs; path: cstring; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.release:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "release%s[%llu] flags: 0x%x\n",
               if fi.flush: "+flush" else: "", cast[culonglong](fi.fh), fi.flags)
    return fs.op.release(path, fi)
  else:
    return 0

proc fuse_fs_opendir*(fs: ptr fuse_fs; path: cstring; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.opendir:
    var err: cint
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "opendir flags: 0x%x %s\n", fi.flags, path)
    err = fs.op.opendir(path, fi)
    if fs.debug and not err:
      fuse_log(FUSE_LOG_DEBUG, "   opendir[%llu] flags: 0x%x %s\n",
               cast[culonglong](fi.fh), fi.flags, path)
    return err
  else:
    return 0

proc fuse_fs_open*(fs: ptr fuse_fs; path: cstring; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.open:
    var err: cint
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "open flags: 0x%x %s\n", fi.flags, path)
    err = fs.op.open(path, fi)
    if fs.debug and not err:
      fuse_log(FUSE_LOG_DEBUG, "   open[%llu] flags: 0x%x %s\n",
               cast[culonglong](fi.fh), fi.flags, path)
    return err
  else:
    return 0

proc fuse_free_buf*(buf: ptr fuse_bufvec) =
  if buf != nil:
    var i: csize_t
    i = 0
    while i < buf.count:
      if not (buf.buf[i].flags and FUSE_BUF_IS_FD):
        free(buf.buf[i].mem)
      inc(i)
    free(buf)

proc fuse_fs_read_buf*(fs: ptr fuse_fs; path: cstring; bufp: ptr ptr fuse_bufvec;
                      size: csize_t; off: off_t; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.read or fs.op.read_buf:
    var res: cint
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "read[%llu] %zu bytes from %llu flags: 0x%x\n",
               cast[culonglong](fi.fh), size, cast[culonglong](off), fi.flags)
    if fs.op.read_buf:
      res = fs.op.read_buf(path, bufp, size, off, fi)
    else:
      var buf: ptr fuse_bufvec
      var mem: pointer
      buf = malloc(sizeof(fuse_bufvec))
      if buf == nil:
        return -ENOMEM
      mem = malloc(size)
      if mem == nil:
        free(buf)
        return -ENOMEM
      buf[] = FUSE_BUFVEC_INIT(size)
      buf.buf[0].mem = mem
      bufp[] = buf
      res = fs.op.read(path, mem, size, off, fi)
      if res >= 0:
        buf.buf[0].size = res
    if fs.debug and res >= 0:
      fuse_log(FUSE_LOG_DEBUG, "   read[%llu] %zu bytes from %llu\n",
               cast[culonglong](fi.fh), fuse_buf_size(bufp[]),
               cast[culonglong](off))
    if res >= 0 and fuse_buf_size(bufp[]) > size:
      fuse_log(FUSE_LOG_ERR, "fuse: read too many bytes\n")
    if res < 0:
      return res
    return 0
  else:
    return -ENOSYS

proc fuse_fs_read*(fs: ptr fuse_fs; path: cstring; mem: cstring; size: csize_t;
                  off: off_t; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.read or fs.op.read_buf:
    var res: cint
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "read[%llu] %zu bytes from %llu flags: 0x%x\n",
               cast[culonglong](fi.fh), size, cast[culonglong](off), fi.flags)
    if fs.op.read_buf:
      var buf: ptr fuse_bufvec = nil
      res = fs.op.read_buf(path, addr(buf), size, off, fi)
      if res == 0:
        var dst: fuse_bufvec = FUSE_BUFVEC_INIT(size)
        dst.buf[0].mem = mem
        res = fuse_buf_copy(addr(dst), buf, 0)
      fuse_free_buf(buf)
    else:
      res = fs.op.read(path, mem, size, off, fi)
    if fs.debug and res >= 0:
      fuse_log(FUSE_LOG_DEBUG, "   read[%llu] %u bytes from %llu\n",
               cast[culonglong](fi.fh), res, cast[culonglong](off))
    if res >= 0 and res > cast[cint](size):
      fuse_log(FUSE_LOG_ERR, "fuse: read too many bytes\n")
    return res
  else:
    return -ENOSYS

proc fuse_fs_write_buf*(fs: ptr fuse_fs; path: cstring; buf: ptr fuse_bufvec; off: off_t;
                       fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.write_buf or fs.op.write:
    var res: cint
    var size: csize_t = fuse_buf_size(buf)
    assert(buf.idx == 0 and buf.off == 0)
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "write%s[%llu] %zu bytes to %llu flags: 0x%x\n",
               if fi.writepage: "page" else: "", cast[culonglong](fi.fh), size,
               cast[culonglong](off), fi.flags)
    if fs.op.write_buf:
      res = fs.op.write_buf(path, buf, off, fi)
    else:
      var mem: pointer = nil
      var flatbuf: ptr fuse_buf
      var tmp: fuse_bufvec = FUSE_BUFVEC_INIT(size)
      if buf.count == 1 and not (buf.buf[0].flags and FUSE_BUF_IS_FD):
        flatbuf = addr(buf.buf[0])
      else:
        res = -ENOMEM
        mem = malloc(size)
        if mem == nil:
          break `out`
        tmp.buf[0].mem = mem
        res = fuse_buf_copy(addr(tmp), buf, 0)
        if res <= 0:
          break out_free
        tmp.buf[0].size = res
        flatbuf = addr(tmp.buf[0])
      res = fs.op.write(path, flatbuf.mem, flatbuf.size, off, fi)
      free(mem)
    if fs.debug and res >= 0:
      fuse_log(FUSE_LOG_DEBUG, "   write%s[%llu] %u bytes to %llu\n",
               if fi.writepage: "page" else: "", cast[culonglong](fi.fh), res,
               cast[culonglong](off))
    if res > cast[cint](size):
      fuse_log(FUSE_LOG_ERR, "fuse: wrote too many bytes\n")
    return res
  else:
    return -ENOSYS

proc fuse_fs_write*(fs: ptr fuse_fs; path: cstring; mem: cstring; size: csize_t;
                   off: off_t; fi: ptr fuse_file_info): cint =
  var bufv: fuse_bufvec = FUSE_BUFVEC_INIT(size)
  bufv.buf[0].mem = cast[pointer](mem)
  return fuse_fs_write_buf(fs, path, addr(bufv), off, fi)

proc fuse_fs_fsync*(fs: ptr fuse_fs; path: cstring; datasync: cint;
                   fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.fsync:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "fsync[%llu] datasync: %i\n",
               cast[culonglong](fi.fh), datasync)
    return fs.op.fsync(path, datasync, fi)
  else:
    return -ENOSYS

proc fuse_fs_fsyncdir*(fs: ptr fuse_fs; path: cstring; datasync: cint;
                      fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.fsyncdir:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "fsyncdir[%llu] datasync: %i\n",
               cast[culonglong](fi.fh), datasync)
    return fs.op.fsyncdir(path, datasync, fi)
  else:
    return -ENOSYS

proc fuse_fs_flush*(fs: ptr fuse_fs; path: cstring; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.flush:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "flush[%llu]\n", cast[culonglong](fi.fh))
    return fs.op.flush(path, fi)
  else:
    return -ENOSYS

proc fuse_fs_statfs*(fs: ptr fuse_fs; path: cstring; buf: ptr statvfs): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.statfs:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "statfs %s\n", path)
    return fs.op.statfs(path, buf)
  else:
    buf.f_namemax = 255
    buf.f_bsize = 512
    return 0

proc fuse_fs_releasedir*(fs: ptr fuse_fs; path: cstring; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.releasedir:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "releasedir[%llu] flags: 0x%x\n",
               cast[culonglong](fi.fh), fi.flags)
    return fs.op.releasedir(path, fi)
  else:
    return 0

proc fuse_fs_readdir*(fs: ptr fuse_fs; path: cstring; buf: pointer;
                     filler: fuse_fill_dir_t; off: off_t; fi: ptr fuse_file_info;
                     flags: fuse_readdir_flags): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.readdir:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "readdir%s[%llu] from %llu\n",
               if (flags and FUSE_READDIR_PLUS): "plus" else: "",
               cast[culonglong](fi.fh), cast[culonglong](off))
    return fs.op.readdir(path, buf, filler, off, fi, flags)
  else:
    return -ENOSYS

proc fuse_fs_create*(fs: ptr fuse_fs; path: cstring; mode: mode_t;
                    fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.create:
    var err: cint
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "create flags: 0x%x %s 0%o umask=0%03o\n",
               fi.flags, path, mode, fuse_get_context().umask)
    err = fs.op.create(path, mode, fi)
    if fs.debug and not err:
      fuse_log(FUSE_LOG_DEBUG, "   create[%llu] flags: 0x%x %s\n",
               cast[culonglong](fi.fh), fi.flags, path)
    return err
  else:
    return -ENOSYS

proc fuse_fs_lock*(fs: ptr fuse_fs; path: cstring; fi: ptr fuse_file_info; cmd: cint;
                  lock: ptr flock): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.lock:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG,
               "lock[%llu] %s %s start: %llu len: %llu pid: %llu\n",
               cast[culonglong](fi.fh), (if cmd == F_GETLK: "F_GETLK" else: (if cmd ==
          F_SETLK: "F_SETLK" else: (if cmd == F_SETLKW: "F_SETLKW" else: "???"))), (if lock.l_type ==
          F_RDLCK: "F_RDLCK" else: (if lock.l_type == F_WRLCK: "F_WRLCK" else: (
          if lock.l_type == F_UNLCK: "F_UNLCK" else: "???"))),
               cast[culonglong](lock.l_start), cast[culonglong](lock.l_len),
               cast[culonglong](lock.l_pid))
    return fs.op.lock(path, fi, cmd, lock)
  else:
    return -ENOSYS

proc fuse_fs_flock*(fs: ptr fuse_fs; path: cstring; fi: ptr fuse_file_info; op: cint): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.flock:
    if fs.debug:
      var xop: cint = op and not LOCK_NB
      fuse_log(FUSE_LOG_DEBUG, "lock[%llu] %s%s\n", cast[culonglong](fi.fh), if xop ==
          LOCK_SH: "LOCK_SH" else: (if xop == LOCK_EX: "LOCK_EX" else: (
          if xop == LOCK_UN: "LOCK_UN" else: "???")),
               if (op and LOCK_NB): "|LOCK_NB" else: "")
    return fs.op.flock(path, fi, op)
  else:
    return -ENOSYS

proc fuse_fs_chown*(fs: ptr fuse_fs; path: cstring; uid: uid_t; gid: gid_t;
                   fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.chown:
    if fs.debug:
      var buf: array[10, char]
      fuse_log(FUSE_LOG_DEBUG, "chown[%s] %s %lu %lu\n",
               file_info_string(fi, buf, sizeof((buf))), path, cast[culong](uid),
               cast[culong](gid))
    return fs.op.chown(path, uid, gid, fi)
  else:
    return -ENOSYS

proc fuse_fs_truncate*(fs: ptr fuse_fs; path: cstring; size: off_t;
                      fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.truncate:
    if fs.debug:
      var buf: array[10, char]
      fuse_log(FUSE_LOG_DEBUG, "truncate[%s] %llu\n",
               file_info_string(fi, buf, sizeof((buf))), cast[culonglong](size))
    return fs.op.truncate(path, size, fi)
  else:
    return -ENOSYS

proc fuse_fs_utimens*(fs: ptr fuse_fs; path: cstring; tv: array[2, timespec];
                     fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.utimens:
    if fs.debug:
      var buf: array[10, char]
      fuse_log(FUSE_LOG_DEBUG, "utimens[%s] %s %li.%09lu %li.%09lu\n",
               file_info_string(fi, buf, sizeof((buf))), path, tv[0].tv_sec,
               tv[0].tv_nsec, tv[1].tv_sec, tv[1].tv_nsec)
    return fs.op.utimens(path, tv, fi)
  else:
    return -ENOSYS

proc fuse_fs_access*(fs: ptr fuse_fs; path: cstring; mask: cint): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.access:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "access %s 0%o\n", path, mask)
    return fs.op.access(path, mask)
  else:
    return -ENOSYS

proc fuse_fs_readlink*(fs: ptr fuse_fs; path: cstring; buf: cstring; len: csize_t): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.readlink:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "readlink %s %lu\n", path, cast[culong](len))
    return fs.op.readlink(path, buf, len)
  else:
    return -ENOSYS

proc fuse_fs_mknod*(fs: ptr fuse_fs; path: cstring; mode: mode_t; rdev: dev_t): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.mknod:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "mknod %s 0%o 0x%llx umask=0%03o\n", path, mode,
               cast[culonglong](rdev), fuse_get_context().umask)
    return fs.op.mknod(path, mode, rdev)
  else:
    return -ENOSYS

proc fuse_fs_mkdir*(fs: ptr fuse_fs; path: cstring; mode: mode_t): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.mkdir:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "mkdir %s 0%o umask=0%03o\n", path, mode,
               fuse_get_context().umask)
    return fs.op.mkdir(path, mode)
  else:
    return -ENOSYS

proc fuse_fs_setxattr*(fs: ptr fuse_fs; path: cstring; name: cstring; value: cstring;
                      size: csize_t; flags: cint): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.setxattr:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "setxattr %s %s %lu 0x%x\n", path, name,
               cast[culong](size), flags)
    return fs.op.setxattr(path, name, value, size, flags)
  else:
    return -ENOSYS

proc fuse_fs_getxattr*(fs: ptr fuse_fs; path: cstring; name: cstring; value: cstring;
                      size: csize_t): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.getxattr:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "getxattr %s %s %lu\n", path, name,
               cast[culong](size))
    return fs.op.getxattr(path, name, value, size)
  else:
    return -ENOSYS

proc fuse_fs_listxattr*(fs: ptr fuse_fs; path: cstring; list: cstring; size: csize_t): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.listxattr:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "listxattr %s %lu\n", path, cast[culong](size))
    return fs.op.listxattr(path, list, size)
  else:
    return -ENOSYS

proc fuse_fs_bmap*(fs: ptr fuse_fs; path: cstring; blocksize: csize_t; idx: ptr uint64_t): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.bmap:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "bmap %s blocksize: %lu index: %llu\n", path,
               cast[culong](blocksize), cast[culonglong](idx[]))
    return fs.op.bmap(path, blocksize, idx)
  else:
    return -ENOSYS

proc fuse_fs_removexattr*(fs: ptr fuse_fs; path: cstring; name: cstring): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.removexattr:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "removexattr %s %s\n", path, name)
    return fs.op.removexattr(path, name)
  else:
    return -ENOSYS

proc fuse_fs_ioctl*(fs: ptr fuse_fs; path: cstring; cmd: cuint; arg: pointer;
                   fi: ptr fuse_file_info; flags: cuint; data: pointer): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.ioctl:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "ioctl[%llu] 0x%x flags: 0x%x\n",
               cast[culonglong](fi.fh), cmd, flags)
    return fs.op.ioctl(path, cmd, arg, fi, flags, data)
  else:
    return -ENOSYS

proc fuse_fs_poll*(fs: ptr fuse_fs; path: cstring; fi: ptr fuse_file_info;
                  ph: ptr fuse_pollhandle; reventsp: ptr cuint): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.poll:
    var res: cint
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG, "poll[%llu] ph: %p, events 0x%x\n",
               cast[culonglong](fi.fh), ph, fi.poll_events)
    res = fs.op.poll(path, fi, ph, reventsp)
    if fs.debug and not res:
      fuse_log(FUSE_LOG_DEBUG, "   poll[%llu] revents: 0x%x\n",
               cast[culonglong](fi.fh), reventsp[])
    return res
  else:
    return -ENOSYS

proc fuse_fs_fallocate*(fs: ptr fuse_fs; path: cstring; mode: cint; offset: off_t;
                       length: off_t; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.fallocate:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG,
               "fallocate %s mode %x, offset: %llu, length: %llu\n", path, mode,
               cast[culonglong](offset), cast[culonglong](length))
    return fs.op.fallocate(path, mode, offset, length, fi)
  else:
    return -ENOSYS

proc fuse_fs_copy_file_range*(fs: ptr fuse_fs; path_in: cstring;
                             fi_in: ptr fuse_file_info; off_in: off_t;
                             path_out: cstring; fi_out: ptr fuse_file_info;
                             off_out: off_t; len: csize_t; flags: cint): ssize_t =
  fuse_get_context().private_data = fs.user_data
  if fs.op.copy_file_range:
    if fs.debug:
      fuse_log(FUSE_LOG_DEBUG,
               "copy_file_range from %s:%llu to %s:%llu, length: %llu\n",
               path_in, cast[culonglong](off_in), path_out,
               cast[culonglong](off_out), cast[culonglong](len))
    return fs.op.copy_file_range(path_in, fi_in, off_in, path_out, fi_out, off_out,
                                len, flags)
  else:
    return -ENOSYS

proc fuse_fs_lseek*(fs: ptr fuse_fs; path: cstring; off: off_t; whence: cint;
                   fi: ptr fuse_file_info): off_t =
  fuse_get_context().private_data = fs.user_data
  if fs.op.lseek:
    if fs.debug:
      var buf: array[10, char]
      fuse_log(FUSE_LOG_DEBUG, "lseek[%s] %llu %d\n",
               file_info_string(fi, buf, sizeof((buf))), cast[culonglong](off),
               whence)
    return fs.op.lseek(path, off, whence, fi)
  else:
    return -ENOSYS

proc is_open*(f: ptr fuse; dir: fuse_ino_t; name: cstring): cint =
  var node: ptr node
  var isopen: cint = 0
  pthread_mutex_lock(addr(f.lock))
  node = lookup_node(f, dir, name)
  if node and node.open_count > 0:
    isopen = 1
  pthread_mutex_unlock(addr(f.lock))
  return isopen

proc hidden_name*(f: ptr fuse; dir: fuse_ino_t; oldname: cstring; newname: cstring;
                 bufsize: csize_t): cstring =
  var buf: stat
  var node: ptr node
  var newnode: ptr node
  var newpath: cstring
  var res: cint
  var failctr: cint = 10
  while true:
    pthread_mutex_lock(addr(f.lock))
    node = lookup_node(f, dir, oldname)
    if node == nil:
      pthread_mutex_unlock(addr(f.lock))
      return nil
    while true:
      inc(f.hidectr)
      snprintf(newname, bufsize, ".fuse_hidden%08x%08x", cast[cuint](node.nodeid),
               f.hidectr)
      newnode = lookup_node(f, dir, newname)
      if not newnode:
        break
    res = try_get_path(f, dir, newname, addr(newpath), nil, false)
    pthread_mutex_unlock(addr(f.lock))
    if res:
      break
    memset(addr(buf), 0, sizeof((buf)))
    res = fuse_fs_getattr(f.fs, newpath, addr(buf), nil)
    if res == -ENOENT:
      break
    free(newpath)
    newpath = nil
    if not (res == 0 and dec(failctr)):
      break
  return newpath

proc hide_node*(f: ptr fuse; oldpath: cstring; dir: fuse_ino_t; oldname: cstring): cint =
  var newname: array[64, char]
  var newpath: cstring
  var err: cint = -EBUSY
  newpath = hidden_name(f, dir, oldname, newname, sizeof((newname)))
  if newpath:
    err = fuse_fs_rename(f.fs, oldpath, newpath, 0)
    if not err:
      err = rename_node(f, dir, oldname, dir, newname, 1)
    free(newpath)
  return err

proc mtime_eq*(stbuf: ptr stat; ts: ptr timespec): cint =
  return stbuf.st_mtime == ts.tv_sec and ST_MTIM_NSEC(stbuf) == ts.tv_nsec

when not defined(CLOCK_MONOTONIC):
  const
    CLOCK_MONOTONIC* = CLOCK_REALTIME
proc curr_time*(now: ptr timespec) =
  var clockid: clockid_t = CLOCK_MONOTONIC
  var res: cint = clock_gettime(clockid, now)
  if res == -1 and errno == EINVAL:
    clockid = CLOCK_REALTIME
    res = clock_gettime(clockid, now)
  if res == -1:
    perror("fuse: clock_gettime")
    abort()

proc update_stat*(node: ptr node; stbuf: ptr stat) =
  if node.cache_valid and
      (not mtime_eq(stbuf, addr(node.mtime)) or stbuf.st_size != node.size):
    node.cache_valid = 0
  node.mtime.tv_sec = stbuf.st_mtime
  node.mtime.tv_nsec = ST_MTIM_NSEC(stbuf)
  node.size = stbuf.st_size
  curr_time(addr(node.stat_updated))

proc do_lookup*(f: ptr fuse; nodeid: fuse_ino_t; name: cstring; e: ptr fuse_entry_param): cint =
  var node: ptr node
  node = find_node(f, nodeid, name)
  if node == nil:
    return -ENOMEM
  e.ino = node.nodeid
  e.generation = node.generation
  e.entry_timeout = f.conf.entry_timeout
  e.attr_timeout = f.conf.attr_timeout
  if f.conf.auto_cache:
    pthread_mutex_lock(addr(f.lock))
    update_stat(node, addr(e.attr))
    pthread_mutex_unlock(addr(f.lock))
  set_stat(f, e.ino, addr(e.attr))
  return 0

proc lookup_path*(f: ptr fuse; nodeid: fuse_ino_t; name: cstring; path: cstring;
                 e: ptr fuse_entry_param; fi: ptr fuse_file_info): cint =
  var res: cint
  memset(e, 0, sizeof(fuse_entry_param))
  res = fuse_fs_getattr(f.fs, path, addr(e.attr), fi)
  if res == 0:
    res = do_lookup(f, nodeid, name, e)
    if res == 0 and f.conf.debug:
      fuse_log(FUSE_LOG_DEBUG, "   NODEID: %llu\n", cast[culonglong](e.ino))
  return res

proc fuse_get_context_internal*(): ptr fuse_context_i =
  return cast[ptr fuse_context_i](pthread_getspecific(fuse_context_key))

proc fuse_create_context*(f: ptr fuse): ptr fuse_context_i =
  var c: ptr fuse_context_i = fuse_get_context_internal()
  if c == nil:
    c = cast[ptr fuse_context_i](calloc(1, sizeof(fuse_context_i)))
    if c == nil:
      ##  This is hard to deal with properly, so just
      ## 			   abort.  If memory is so low that the
      ## 			   context cannot be allocated, there's not
      ## 			   much hope for the filesystem anyway
      fuse_log(FUSE_LOG_ERR, "fuse: failed to allocate thread specific data\n")
      abort()
    pthread_setspecific(fuse_context_key, c)
  else:
    memset(c, 0, sizeof((c[])))
  c.ctx.fuse = f
  return c

proc fuse_freecontext*(data: pointer) =
  free(data)

proc fuse_create_context_key*(): cint =
  var err: cint = 0
  pthread_mutex_lock(addr(fuse_context_lock))
  if not fuse_context_ref:
    err = pthread_key_create(addr(fuse_context_key), fuse_freecontext)
    if err:
      fuse_log(FUSE_LOG_ERR, "fuse: failed to create thread specific key: %s\n",
               strerror(err))
      pthread_mutex_unlock(addr(fuse_context_lock))
      return -1
  inc(fuse_context_ref)
  pthread_mutex_unlock(addr(fuse_context_lock))
  return 0

proc fuse_delete_context_key*() =
  pthread_mutex_lock(addr(fuse_context_lock))
  dec(fuse_context_ref)
  if not fuse_context_ref:
    free(pthread_getspecific(fuse_context_key))
    pthread_key_delete(fuse_context_key)
  pthread_mutex_unlock(addr(fuse_context_lock))

proc req_fuse_prepare*(req: fuse_req_t): ptr fuse =
  var c: ptr fuse_context_i = fuse_create_context(req_fuse(req))
  var ctx: ptr fuse_ctx = fuse_req_ctx(req)
  c.req = req
  c.ctx.uid = ctx.uid
  c.ctx.gid = ctx.gid
  c.ctx.pid = ctx.pid
  c.ctx.umask = ctx.umask
  return c.ctx.fuse

proc reply_err*(req: fuse_req_t; err: cint) {.inline.} =
  ##  fuse_reply_err() uses non-negated errno values
  fuse_reply_err(req, -err)

proc reply_entry*(req: fuse_req_t; e: ptr fuse_entry_param; err: cint) =
  if not err:
    var f: ptr fuse = req_fuse(req)
    if fuse_reply_entry(req, e) == -ENOENT:
      ##  Skip forget for negative result
      if e.ino != 0:
        forget_node(f, e.ino, 1)
  else:
    reply_err(req, err)

proc fuse_fs_init*(fs: ptr fuse_fs; conn: ptr fuse_conn_info; cfg: ptr fuse_config) =
  fuse_get_context().private_data = fs.user_data
  if not fs.op.write_buf:
    conn.want = conn.want and not FUSE_CAP_SPLICE_READ
  if not fs.op.lock:
    conn.want = conn.want and not FUSE_CAP_POSIX_LOCKS
  if not fs.op.flock:
    conn.want = conn.want and not FUSE_CAP_FLOCK_LOCKS
  if fs.op.init:
    fs.user_data = fs.op.init(conn, cfg)

proc fuse_lib_init*(data: pointer; conn: ptr fuse_conn_info) =
  var f: ptr fuse = cast[ptr fuse](data)
  fuse_create_context(f)
  if conn.capable and FUSE_CAP_EXPORT_SUPPORT:
    conn.want = conn.want or FUSE_CAP_EXPORT_SUPPORT
  fuse_fs_init(f.fs, conn, addr(f.conf))

proc fuse_fs_destroy*(fs: ptr fuse_fs) =
  fuse_get_context().private_data = fs.user_data
  if fs.op.destroy:
    fs.op.destroy(fs.user_data)
  if fs.m:
    fuse_put_module(fs.m)
  free(fs)

proc fuse_lib_destroy*(data: pointer) =
  var f: ptr fuse = cast[ptr fuse](data)
  fuse_create_context(f)
  fuse_fs_destroy(f.fs)
  f.fs = nil

proc fuse_lib_lookup*(req: fuse_req_t; parent: fuse_ino_t; name: cstring) =
  var f: ptr fuse = req_fuse_prepare(req)
  var e: fuse_entry_param
  var path: cstring
  var err: cint
  var dot: ptr node = nil
  if name[0] == '.':
    var len: cint = strlen(name)
    if len == 1 or (name[1] == '.' and len == 2):
      pthread_mutex_lock(addr(f.lock))
      if len == 1:
        if f.conf.debug:
          fuse_log(FUSE_LOG_DEBUG, "LOOKUP-DOT\n")
        dot = get_node_nocheck(f, parent)
        if dot == nil:
          pthread_mutex_unlock(addr(f.lock))
          reply_entry(req, addr(e), -ESTALE)
          return
        inc(dot.refctr)
      else:
        if f.conf.debug:
          fuse_log(FUSE_LOG_DEBUG, "LOOKUP-DOTDOT\n")
        parent = get_node(f, parent).parent.nodeid
      pthread_mutex_unlock(addr(f.lock))
      name = nil
  err = get_path_name(f, parent, name, addr(path))
  if not err:
    var d: fuse_intr_data
    if f.conf.debug:
      fuse_log(FUSE_LOG_DEBUG, "LOOKUP %s\n", path)
    fuse_prepare_interrupt(f, req, addr(d))
    err = lookup_path(f, parent, name, path, addr(e), nil)
    if err == -ENOENT and f.conf.negative_timeout != 0.0:
      e.ino = 0
      e.entry_timeout = f.conf.negative_timeout
      err = 0
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, parent, path)
  if dot:
    pthread_mutex_lock(addr(f.lock))
    unref_node(f, dot)
    pthread_mutex_unlock(addr(f.lock))
  reply_entry(req, addr(e), err)

proc do_forget*(f: ptr fuse; ino: fuse_ino_t; nlookup: uint64_t) =
  if f.conf.debug:
    fuse_log(FUSE_LOG_DEBUG, "FORGET %llu/%llu\n", cast[culonglong](ino),
             cast[culonglong](nlookup))
  forget_node(f, ino, nlookup)

proc fuse_lib_forget*(req: fuse_req_t; ino: fuse_ino_t; nlookup: uint64_t) =
  do_forget(req_fuse(req), ino, nlookup)
  fuse_reply_none(req)

proc fuse_lib_forget_multi*(req: fuse_req_t; count: csize_t;
                           forgets: ptr fuse_forget_data) =
  var f: ptr fuse = req_fuse(req)
  var i: csize_t
  i = 0
  while i < count:
    do_forget(f, forgets[i].ino, forgets[i].nlookup)
    inc(i)
  fuse_reply_none(req)

proc fuse_lib_getattr*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var buf: stat
  var path: cstring
  var err: cint
  memset(addr(buf), 0, sizeof((buf)))
  if fi != nil:
    err = get_path_nullok(f, ino, addr(path))
  else:
    err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_getattr(f.fs, path, addr(buf), fi)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  if not err:
    var node: ptr node
    pthread_mutex_lock(addr(f.lock))
    node = get_node(f, ino)
    if node.is_hidden and buf.st_nlink > 0:
      dec(buf.st_nlink)
    if f.conf.auto_cache:
      update_stat(node, addr(buf))
    pthread_mutex_unlock(addr(f.lock))
    set_stat(f, ino, addr(buf))
    fuse_reply_attr(req, addr(buf), f.conf.attr_timeout)
  else:
    reply_err(req, err)

proc fuse_fs_chmod*(fs: ptr fuse_fs; path: cstring; mode: mode_t; fi: ptr fuse_file_info): cint =
  fuse_get_context().private_data = fs.user_data
  if fs.op.chmod:
    if fs.debug:
      var buf: array[10, char]
      fuse_log(FUSE_LOG_DEBUG, "chmod[%s] %s %llo\n",
               file_info_string(fi, buf, sizeof((buf))), path,
               cast[culonglong](mode))
    return fs.op.chmod(path, mode, fi)
  else:
    return -ENOSYS

proc fuse_lib_setattr*(req: fuse_req_t; ino: fuse_ino_t; attr: ptr stat; valid: cint;
                      fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var buf: stat
  var path: cstring
  var err: cint
  memset(addr(buf), 0, sizeof((buf)))
  if fi != nil:
    err = get_path_nullok(f, ino, addr(path))
  else:
    err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = 0
    if not err and (valid and FUSE_SET_ATTR_MODE):
      err = fuse_fs_chmod(f.fs, path, attr.st_mode, fi)
    if not err and (valid and (FUSE_SET_ATTR_UID or FUSE_SET_ATTR_GID)):
      var uid: uid_t = if (valid and FUSE_SET_ATTR_UID): attr.st_uid else: (uid_t) - 1
      var gid: gid_t = if (valid and FUSE_SET_ATTR_GID): attr.st_gid else: (gid_t) - 1
      err = fuse_fs_chown(f.fs, path, uid, gid, fi)
    if not err and (valid and FUSE_SET_ATTR_SIZE):
      err = fuse_fs_truncate(f.fs, path, attr.st_size, fi)
    ## !!!Ignored construct:  # HAVE_UTIMENSAT [NewLine] if ( ! err && ( valid & ( FUSE_SET_ATTR_ATIME | FUSE_SET_ATTR_MTIME ) ) ) { struct timespec tv [ 2 ] ; tv [ 0 ] . tv_sec = 0 ; tv [ 1 ] . tv_sec = 0 ; tv [ 0 ] . tv_nsec = UTIME_OMIT ; tv [ 1 ] . tv_nsec = UTIME_OMIT ; if ( valid & FUSE_SET_ATTR_ATIME_NOW ) tv [ 0 ] . tv_nsec = UTIME_NOW ; else if ( valid & FUSE_SET_ATTR_ATIME ) tv [ 0 ] = attr -> st_atim ; if ( valid & FUSE_SET_ATTR_MTIME_NOW ) tv [ 1 ] . tv_nsec = UTIME_NOW ; else if ( valid & FUSE_SET_ATTR_MTIME ) tv [ 1 ] = attr -> st_mtim ; err = fuse_fs_utimens ( f -> fs , path , tv , fi ) ; } else # [NewLine] if ( ! err && ( valid & ( FUSE_SET_ATTR_ATIME | FUSE_SET_ATTR_MTIME ) ) == ( FUSE_SET_ATTR_ATIME | FUSE_SET_ATTR_MTIME ) ) { struct timespec tv [ 2 ] ; tv [ 0 ] . tv_sec = attr -> st_atime ; tv [ 0 ] . tv_nsec = ST_ATIM_NSEC ( attr ) ; tv [ 1 ] . tv_sec = attr -> st_mtime ; tv [ 1 ] . tv_nsec = ST_MTIM_NSEC ( attr ) ; err = fuse_fs_utimens ( f -> fs , path , tv , fi ) ; } if ( ! err ) { err = fuse_fs_getattr ( f -> fs , path , & buf , fi ) ; } fuse_finish_interrupt ( f , req , & d ) ;
    ## Error: did not expect }!!!
    free_path(f, ino, path)
  if not err:
    if f.conf.auto_cache:
      pthread_mutex_lock(addr(f.lock))
      update_stat(get_node(f, ino), addr(buf))
      pthread_mutex_unlock(addr(f.lock))
    set_stat(f, ino, addr(buf))
    fuse_reply_attr(req, addr(buf), f.conf.attr_timeout)
  else:
    reply_err(req, err)

proc fuse_lib_access*(req: fuse_req_t; ino: fuse_ino_t; mask: cint) =
  var f: ptr fuse = req_fuse_prepare(req)
  var path: cstring
  var err: cint
  err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_access(f.fs, path, mask)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  reply_err(req, err)

proc fuse_lib_readlink*(req: fuse_req_t; ino: fuse_ino_t) =
  var f: ptr fuse = req_fuse_prepare(req)
  var linkname: array[PATH_MAX + 1, char]
  var path: cstring
  var err: cint
  err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_readlink(f.fs, path, linkname, sizeof((linkname)))
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  if not err:
    linkname[PATH_MAX] = '\x00'
    fuse_reply_readlink(req, linkname)
  else:
    reply_err(req, err)

proc fuse_lib_mknod*(req: fuse_req_t; parent: fuse_ino_t; name: cstring; mode: mode_t;
                    rdev: dev_t) =
  var f: ptr fuse = req_fuse_prepare(req)
  var e: fuse_entry_param
  var path: cstring
  var err: cint
  err = get_path_name(f, parent, name, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = -ENOSYS
    if S_ISREG(mode):
      var fi: fuse_file_info
      memset(addr(fi), 0, sizeof((fi)))
      fi.flags = O_CREAT or O_EXCL or O_WRONLY
      err = fuse_fs_create(f.fs, path, mode, addr(fi))
      if not err:
        err = lookup_path(f, parent, name, path, addr(e), addr(fi))
        fuse_fs_release(f.fs, path, addr(fi))
    if err == -ENOSYS:
      err = fuse_fs_mknod(f.fs, path, mode, rdev)
      if not err:
        err = lookup_path(f, parent, name, path, addr(e), nil)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, parent, path)
  reply_entry(req, addr(e), err)

proc fuse_lib_mkdir*(req: fuse_req_t; parent: fuse_ino_t; name: cstring; mode: mode_t) =
  var f: ptr fuse = req_fuse_prepare(req)
  var e: fuse_entry_param
  var path: cstring
  var err: cint
  err = get_path_name(f, parent, name, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_mkdir(f.fs, path, mode)
    if not err:
      err = lookup_path(f, parent, name, path, addr(e), nil)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, parent, path)
  reply_entry(req, addr(e), err)

proc fuse_lib_unlink*(req: fuse_req_t; parent: fuse_ino_t; name: cstring) =
  var f: ptr fuse = req_fuse_prepare(req)
  var wnode: ptr node
  var path: cstring
  var err: cint
  err = get_path_wrlock(f, parent, name, addr(path), addr(wnode))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    if not f.conf.hard_remove and is_open(f, parent, name):
      err = hide_node(f, path, parent, name)
    else:
      err = fuse_fs_unlink(f.fs, path)
      if not err:
        remove_node(f, parent, name)
    fuse_finish_interrupt(f, req, addr(d))
    free_path_wrlock(f, parent, wnode, path)
  reply_err(req, err)

proc fuse_lib_rmdir*(req: fuse_req_t; parent: fuse_ino_t; name: cstring) =
  var f: ptr fuse = req_fuse_prepare(req)
  var wnode: ptr node
  var path: cstring
  var err: cint
  err = get_path_wrlock(f, parent, name, addr(path), addr(wnode))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_rmdir(f.fs, path)
    fuse_finish_interrupt(f, req, addr(d))
    if not err:
      remove_node(f, parent, name)
    free_path_wrlock(f, parent, wnode, path)
  reply_err(req, err)

proc fuse_lib_symlink*(req: fuse_req_t; linkname: cstring; parent: fuse_ino_t;
                      name: cstring) =
  var f: ptr fuse = req_fuse_prepare(req)
  var e: fuse_entry_param
  var path: cstring
  var err: cint
  err = get_path_name(f, parent, name, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_symlink(f.fs, linkname, path)
    if not err:
      err = lookup_path(f, parent, name, path, addr(e), nil)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, parent, path)
  reply_entry(req, addr(e), err)

proc fuse_lib_rename*(req: fuse_req_t; olddir: fuse_ino_t; oldname: cstring;
                     newdir: fuse_ino_t; newname: cstring; flags: cuint) =
  var f: ptr fuse = req_fuse_prepare(req)
  var oldpath: cstring
  var newpath: cstring
  var wnode1: ptr node
  var wnode2: ptr node
  var err: cint
  err = get_path2(f, olddir, oldname, newdir, newname, addr(oldpath), addr(newpath),
                addr(wnode1), addr(wnode2))
  if not err:
    var d: fuse_intr_data
    err = 0
    fuse_prepare_interrupt(f, req, addr(d))
    if not f.conf.hard_remove and not (flags and RENAME_EXCHANGE) and
        is_open(f, newdir, newname):
      err = hide_node(f, newpath, newdir, newname)
    if not err:
      err = fuse_fs_rename(f.fs, oldpath, newpath, flags)
      if not err:
        if flags and RENAME_EXCHANGE:
          err = exchange_node(f, olddir, oldname, newdir, newname)
        else:
          err = rename_node(f, olddir, oldname, newdir, newname, 0)
    fuse_finish_interrupt(f, req, addr(d))
    free_path2(f, olddir, newdir, wnode1, wnode2, oldpath, newpath)
  reply_err(req, err)

proc fuse_lib_link*(req: fuse_req_t; ino: fuse_ino_t; newparent: fuse_ino_t;
                   newname: cstring) =
  var f: ptr fuse = req_fuse_prepare(req)
  var e: fuse_entry_param
  var oldpath: cstring
  var newpath: cstring
  var err: cint
  err = get_path2(f, ino, nil, newparent, newname, addr(oldpath), addr(newpath), nil, nil)
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_link(f.fs, oldpath, newpath)
    if not err:
      err = lookup_path(f, newparent, newname, newpath, addr(e), nil)
    fuse_finish_interrupt(f, req, addr(d))
    free_path2(f, ino, newparent, nil, nil, oldpath, newpath)
  reply_entry(req, addr(e), err)

proc fuse_do_release*(f: ptr fuse; ino: fuse_ino_t; path: cstring;
                     fi: ptr fuse_file_info) =
  var node: ptr node
  var unlink_hidden: cint = 0
  fuse_fs_release(f.fs, path, fi)
  pthread_mutex_lock(addr(f.lock))
  node = get_node(f, ino)
  assert(node.open_count > 0)
  dec(node.open_count)
  if node.is_hidden and not node.open_count:
    unlink_hidden = 1
    node.is_hidden = 0
  pthread_mutex_unlock(addr(f.lock))
  if unlink_hidden:
    if path:
      fuse_fs_unlink(f.fs, path)
    elif f.conf.nullpath_ok:
      var unlinkpath: cstring
      if get_path(f, ino, addr(unlinkpath)) == 0:
        fuse_fs_unlink(f.fs, unlinkpath)
      free_path(f, ino, unlinkpath)

proc fuse_lib_create*(req: fuse_req_t; parent: fuse_ino_t; name: cstring; mode: mode_t;
                     fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var e: fuse_entry_param
  var path: cstring
  var err: cint
  err = get_path_name(f, parent, name, addr(path))
  if not err:
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_create(f.fs, path, mode, fi)
    if not err:
      err = lookup_path(f, parent, name, path, addr(e), fi)
      if err:
        fuse_fs_release(f.fs, path, fi)
      elif not S_ISREG(e.attr.st_mode):
        err = -EIO
        fuse_fs_release(f.fs, path, fi)
        forget_node(f, e.ino, 1)
      else:
        if f.conf.direct_io:
          fi.direct_io = 1
        if f.conf.kernel_cache:
          fi.keep_cache = 1
    fuse_finish_interrupt(f, req, addr(d))
  if not err:
    pthread_mutex_lock(addr(f.lock))
    inc(get_node(f, e.ino).open_count)
    pthread_mutex_unlock(addr(f.lock))
    if fuse_reply_create(req, addr(e), fi) == -ENOENT:
      ##  The open syscall was interrupted, so it
      ## 			   must be cancelled
      fuse_do_release(f, e.ino, path, fi)
      forget_node(f, e.ino, 1)
  else:
    reply_err(req, err)
  free_path(f, parent, path)

proc diff_timespec*(t1: ptr timespec; t2: ptr timespec): cdouble =
  return (t1.tv_sec - t2.tv_sec) +
      (cast[cdouble](t1.tv_nsec) - cast[cdouble](t2.tv_nsec)) div 1000000000.0

proc open_auto_cache*(f: ptr fuse; ino: fuse_ino_t; path: cstring;
                     fi: ptr fuse_file_info) =
  var node: ptr node
  pthread_mutex_lock(addr(f.lock))
  node = get_node(f, ino)
  if node.cache_valid:
    var now: timespec
    curr_time(addr(now))
    if diff_timespec(addr(now), addr(node.stat_updated)) > f.conf.ac_attr_timeout:
      var stbuf: stat
      var err: cint
      pthread_mutex_unlock(addr(f.lock))
      err = fuse_fs_getattr(f.fs, path, addr(stbuf), fi)
      pthread_mutex_lock(addr(f.lock))
      if not err:
        update_stat(node, addr(stbuf))
      else:
        node.cache_valid = 0
  if node.cache_valid:
    fi.keep_cache = 1
  node.cache_valid = 1
  pthread_mutex_unlock(addr(f.lock))

proc fuse_lib_open*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var path: cstring
  var err: cint
  err = get_path(f, ino, addr(path))
  if not err:
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_open(f.fs, path, fi)
    if not err:
      if f.conf.direct_io:
        fi.direct_io = 1
      if f.conf.kernel_cache:
        fi.keep_cache = 1
      if f.conf.auto_cache:
        open_auto_cache(f, ino, path, fi)
      if f.conf.no_rofd_flush and (fi.flags and O_ACCMODE) == O_RDONLY:
        fi.noflush = 1
    fuse_finish_interrupt(f, req, addr(d))
  if not err:
    pthread_mutex_lock(addr(f.lock))
    inc(get_node(f, ino).open_count)
    pthread_mutex_unlock(addr(f.lock))
    if fuse_reply_open(req, fi) == -ENOENT:
      ##  The open syscall was interrupted, so it
      ## 			   must be cancelled
      fuse_do_release(f, ino, path, fi)
  else:
    reply_err(req, err)
  free_path(f, ino, path)

proc fuse_lib_read*(req: fuse_req_t; ino: fuse_ino_t; size: csize_t; off: off_t;
                   fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var buf: ptr fuse_bufvec = nil
  var path: cstring
  var res: cint
  res = get_path_nullok(f, ino, addr(path))
  if res == 0:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    res = fuse_fs_read_buf(f.fs, path, addr(buf), size, off, fi)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  if res == 0:
    fuse_reply_data(req, buf, FUSE_BUF_SPLICE_MOVE)
  else:
    reply_err(req, res)
  fuse_free_buf(buf)

proc fuse_lib_write_buf*(req: fuse_req_t; ino: fuse_ino_t; buf: ptr fuse_bufvec;
                        off: off_t; fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var path: cstring
  var res: cint
  res = get_path_nullok(f, ino, addr(path))
  if res == 0:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    res = fuse_fs_write_buf(f.fs, path, buf, off, fi)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  if res >= 0:
    fuse_reply_write(req, res)
  else:
    reply_err(req, res)

proc fuse_lib_fsync*(req: fuse_req_t; ino: fuse_ino_t; datasync: cint;
                    fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var path: cstring
  var err: cint
  err = get_path_nullok(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_fsync(f.fs, path, datasync, fi)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  reply_err(req, err)

proc get_dirhandle*(llfi: ptr fuse_file_info; fi: ptr fuse_file_info): ptr fuse_dh =
  var dh: ptr fuse_dh = cast[ptr fuse_dh](cast[uintptr_t](llfi.fh))
  memset(fi, 0, sizeof(fuse_file_info))
  fi.fh = dh.fh
  return dh

proc fuse_lib_opendir*(req: fuse_req_t; ino: fuse_ino_t; llfi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var dh: ptr fuse_dh
  var fi: fuse_file_info
  var path: cstring
  var err: cint
  dh = cast[ptr fuse_dh](malloc(sizeof(fuse_dh)))
  if dh == nil:
    reply_err(req, -ENOMEM)
    return
  memset(dh, 0, sizeof(fuse_dh))
  dh.fuse = f
  dh.contents = nil
  dh.first = nil
  dh.len = 0
  dh.filled = 0
  dh.nodeid = ino
  pthread_mutex_init(addr(dh.lock), nil)
  llfi.fh = cast[uintptr_t](dh)
  memset(addr(fi), 0, sizeof((fi)))
  fi.flags = llfi.flags
  err = get_path(f, ino, addr(path))
  if not err:
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_opendir(f.fs, path, addr(fi))
    fuse_finish_interrupt(f, req, addr(d))
    dh.fh = fi.fh
  if not err:
    if fuse_reply_open(req, llfi) == -ENOENT:
      ##  The opendir syscall was interrupted, so it
      ## 			   must be cancelled
      fuse_fs_releasedir(f.fs, path, addr(fi))
      pthread_mutex_destroy(addr(dh.lock))
      free(dh)
  else:
    reply_err(req, err)
    pthread_mutex_destroy(addr(dh.lock))
    free(dh)
  free_path(f, ino, path)

proc extend_contents*(dh: ptr fuse_dh; minsize: cuint): cint =
  if minsize > dh.size:
    var newptr: cstring
    var newsize: cuint = dh.size
    if not newsize:
      newsize = 1024
    while newsize < minsize:
      if newsize >= 0x80000000:
        newsize = 0xffffffff
      else:
        newsize = newsize * 2
    newptr = cast[cstring](realloc(dh.contents, newsize))
    if not newptr:
      dh.error = -ENOMEM
      return -1
    dh.contents = newptr
    dh.size = newsize
  return 0

proc fuse_add_direntry_to_dh*(dh: ptr fuse_dh; name: cstring; st: ptr stat): cint =
  var de: ptr fuse_direntry
  de = malloc(sizeof(fuse_direntry))
  if not de:
    dh.error = -ENOMEM
    return -1
  de.name = strdup(name)
  if not de.name:
    dh.error = -ENOMEM
    free(de)
    return -1
  de.stat = st[]
  de.next = nil
  dh.last[] = de
  dh.last = addr(de.next)
  return 0

proc lookup_nodeid*(f: ptr fuse; parent: fuse_ino_t; name: cstring): fuse_ino_t =
  var node: ptr node
  var res: fuse_ino_t = FUSE_UNKNOWN_INO
  pthread_mutex_lock(addr(f.lock))
  node = lookup_node(f, parent, name)
  if node:
    res = node.nodeid
  pthread_mutex_unlock(addr(f.lock))
  return res

proc fill_dir*(dh_: pointer; name: cstring; statp: ptr stat; off: off_t;
              flags: fuse_fill_dir_flags): cint =
  var dh: ptr fuse_dh = cast[ptr fuse_dh](dh_)
  var stbuf: stat
  if (flags and not FUSE_FILL_DIR_PLUS) != 0:
    dh.error = -EIO
    return 1
  if statp:
    stbuf = statp[]
  else:
    memset(addr(stbuf), 0, sizeof((stbuf)))
    stbuf.st_ino = FUSE_UNKNOWN_INO
  if not dh.fuse.conf.use_ino:
    stbuf.st_ino = FUSE_UNKNOWN_INO
    if dh.fuse.conf.readdir_ino:
      stbuf.st_ino = cast[ino_t](lookup_nodeid(dh.fuse, dh.nodeid, name))
  if off:
    var newlen: csize_t
    if dh.filled:
      dh.error = -EIO
      return 1
    if dh.first:
      dh.error = -EIO
      return 1
    if extend_contents(dh, dh.needlen) == -1:
      return 1
    newlen = dh.len +
        fuse_add_direntry(dh.req, dh.contents + dh.len, dh.needlen - dh.len, name,
                          addr(stbuf), off)
    if newlen > dh.needlen:
      return 1
    dh.len = newlen
  else:
    dh.filled = 1
    if fuse_add_direntry_to_dh(dh, name, addr(stbuf)) == -1:
      return 1
  return 0

proc is_dot_or_dotdot*(name: cstring): cint =
  return name[0] == '.' and
      (name[1] == '\x00' or (name[1] == '.' and name[2] == '\x00'))

proc fill_dir_plus*(dh_: pointer; name: cstring; statp: ptr stat; off: off_t;
                   flags: fuse_fill_dir_flags): cint =
  var dh: ptr fuse_dh = cast[ptr fuse_dh](dh_)
  var e: fuse_entry_param = [ino: 0] ##  ino=0 tells the kernel to ignore readdirplus stat info
  var f: ptr fuse = dh.fuse
  var res: cint
  if (flags and not FUSE_FILL_DIR_PLUS) != 0:
    dh.error = -EIO
    return 1
  if statp and (flags and FUSE_FILL_DIR_PLUS):
    e.attr = statp[]
    if not is_dot_or_dotdot(name):
      res = do_lookup(f, dh.nodeid, name, addr(e))
      if res:
        dh.error = res
        return 1
  else:
    e.attr.st_ino = FUSE_UNKNOWN_INO
    if statp:
      e.attr.st_mode = statp.st_mode
      if f.conf.use_ino:
        e.attr.st_ino = statp.st_ino
    if not f.conf.use_ino and f.conf.readdir_ino:
      e.attr.st_ino = cast[ino_t](lookup_nodeid(f, dh.nodeid, name))
  if off:
    var newlen: csize_t
    if dh.filled:
      dh.error = -EIO
      return 1
    if dh.first:
      dh.error = -EIO
      return 1
    if extend_contents(dh, dh.needlen) == -1:
      return 1
    newlen = dh.len +
        fuse_add_direntry_plus(dh.req, dh.contents + dh.len, dh.needlen - dh.len, name,
                               addr(e), off)
    if newlen > dh.needlen:
      return 1
    dh.len = newlen
  else:
    dh.filled = 1
    if fuse_add_direntry_to_dh(dh, name, addr(e.attr)) == -1:
      return 1
  return 0

proc free_direntries*(de: ptr fuse_direntry) =
  while de:
    var next: ptr fuse_direntry = de.next
    free(de.name)
    free(de)
    de = next

proc readdir_fill*(f: ptr fuse; req: fuse_req_t; ino: fuse_ino_t; size: csize_t;
                  off: off_t; dh: ptr fuse_dh; fi: ptr fuse_file_info;
                  flags: fuse_readdir_flags): cint =
  var path: cstring
  var err: cint
  if f.fs.op.readdir:
    err = get_path_nullok(f, ino, addr(path))
  else:
    err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    var filler: fuse_fill_dir_t = fill_dir
    if flags and FUSE_READDIR_PLUS:
      filler = fill_dir_plus
    free_direntries(dh.first)
    dh.first = nil
    dh.last = addr(dh.first)
    dh.len = 0
    dh.error = 0
    dh.needlen = size
    dh.filled = 0
    dh.req = req
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_readdir(f.fs, path, dh, filler, off, fi, flags)
    fuse_finish_interrupt(f, req, addr(d))
    dh.req = nil
    if not err:
      err = dh.error
    if err:
      dh.filled = 0
    free_path(f, ino, path)
  return err

proc readdir_fill_from_list*(req: fuse_req_t; dh: ptr fuse_dh; off: off_t;
                            flags: fuse_readdir_flags): cint =
  var pos: off_t
  var de: ptr fuse_direntry = dh.first
  dh.len = 0
  if extend_contents(dh, dh.needlen) == -1:
    return dh.error
  pos = 0
  while pos < off:
    if not de:
      break
    de = de.next
    inc(pos)
  while de:
    var p: cstring = dh.contents + dh.len
    var rem: cuint = dh.needlen - dh.len
    var thislen: cuint
    var newlen: cuint
    inc(pos)
    if flags and FUSE_READDIR_PLUS:
      var e: fuse_entry_param = [ino: 0, attr: de.stat]
      thislen = fuse_add_direntry_plus(req, p, rem, de.name, addr(e), pos)
    else:
      thislen = fuse_add_direntry(req, p, rem, de.name, addr(de.stat), pos)
    newlen = dh.len + thislen
    if newlen > dh.needlen:
      break
    dh.len = newlen
    de = de.next
  return 0

proc fuse_readdir_common*(req: fuse_req_t; ino: fuse_ino_t; size: csize_t; off: off_t;
                         llfi: ptr fuse_file_info; flags: fuse_readdir_flags) =
  var f: ptr fuse = req_fuse_prepare(req)
  var fi: fuse_file_info
  var dh: ptr fuse_dh = get_dirhandle(llfi, addr(fi))
  var err: cint
  pthread_mutex_lock(addr(dh.lock))
  ##  According to SUS, directory contents need to be refreshed on
  ## 	   rewinddir()
  if not off:
    dh.filled = 0
  if not dh.filled:
    err = readdir_fill(f, req, ino, size, off, dh, addr(fi), flags)
    if err:
      reply_err(req, err)
      break `out`
  if dh.filled:
    dh.needlen = size
    err = readdir_fill_from_list(req, dh, off, flags)
    if err:
      reply_err(req, err)
      break `out`
  fuse_reply_buf(req, dh.contents, dh.len)
  pthread_mutex_unlock(addr(dh.lock))

proc fuse_lib_readdir*(req: fuse_req_t; ino: fuse_ino_t; size: csize_t; off: off_t;
                      llfi: ptr fuse_file_info) =
  fuse_readdir_common(req, ino, size, off, llfi, 0)

proc fuse_lib_readdirplus*(req: fuse_req_t; ino: fuse_ino_t; size: csize_t; off: off_t;
                          llfi: ptr fuse_file_info) =
  fuse_readdir_common(req, ino, size, off, llfi, FUSE_READDIR_PLUS)

proc fuse_lib_releasedir*(req: fuse_req_t; ino: fuse_ino_t; llfi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var fi: fuse_file_info
  var dh: ptr fuse_dh = get_dirhandle(llfi, addr(fi))
  var path: cstring
  get_path_nullok(f, ino, addr(path))
  fuse_prepare_interrupt(f, req, addr(d))
  fuse_fs_releasedir(f.fs, path, addr(fi))
  fuse_finish_interrupt(f, req, addr(d))
  free_path(f, ino, path)
  pthread_mutex_lock(addr(dh.lock))
  pthread_mutex_unlock(addr(dh.lock))
  pthread_mutex_destroy(addr(dh.lock))
  free_direntries(dh.first)
  free(dh.contents)
  free(dh)
  reply_err(req, 0)

proc fuse_lib_fsyncdir*(req: fuse_req_t; ino: fuse_ino_t; datasync: cint;
                       llfi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var fi: fuse_file_info
  var path: cstring
  var err: cint
  get_dirhandle(llfi, addr(fi))
  err = get_path_nullok(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_fsyncdir(f.fs, path, datasync, addr(fi))
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  reply_err(req, err)

proc fuse_lib_statfs*(req: fuse_req_t; ino: fuse_ino_t) =
  var f: ptr fuse = req_fuse_prepare(req)
  var buf: statvfs
  var path: cstring = nil
  var err: cint = 0
  memset(addr(buf), 0, sizeof((buf)))
  if ino:
    err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_statfs(f.fs, if path: path else: "/", addr(buf))
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  if not err:
    fuse_reply_statfs(req, addr(buf))
  else:
    reply_err(req, err)

proc fuse_lib_setxattr*(req: fuse_req_t; ino: fuse_ino_t; name: cstring;
                       value: cstring; size: csize_t; flags: cint) =
  var f: ptr fuse = req_fuse_prepare(req)
  var path: cstring
  var err: cint
  err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_setxattr(f.fs, path, name, value, size, flags)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  reply_err(req, err)

proc common_getxattr*(f: ptr fuse; req: fuse_req_t; ino: fuse_ino_t; name: cstring;
                     value: cstring; size: csize_t): cint =
  var err: cint
  var path: cstring
  err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_getxattr(f.fs, path, name, value, size)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  return err

proc fuse_lib_getxattr*(req: fuse_req_t; ino: fuse_ino_t; name: cstring; size: csize_t) =
  var f: ptr fuse = req_fuse_prepare(req)
  var res: cint
  if size:
    var value: cstring = cast[cstring](malloc(size))
    if value == nil:
      reply_err(req, -ENOMEM)
      return
    res = common_getxattr(f, req, ino, name, value, size)
    if res > 0:
      fuse_reply_buf(req, value, res)
    else:
      reply_err(req, res)
    free(value)
  else:
    res = common_getxattr(f, req, ino, name, nil, 0)
    if res >= 0:
      fuse_reply_xattr(req, res)
    else:
      reply_err(req, res)

proc common_listxattr*(f: ptr fuse; req: fuse_req_t; ino: fuse_ino_t; list: cstring;
                      size: csize_t): cint =
  var path: cstring
  var err: cint
  err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_listxattr(f.fs, path, list, size)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  return err

proc fuse_lib_listxattr*(req: fuse_req_t; ino: fuse_ino_t; size: csize_t) =
  var f: ptr fuse = req_fuse_prepare(req)
  var res: cint
  if size:
    var list: cstring = cast[cstring](malloc(size))
    if list == nil:
      reply_err(req, -ENOMEM)
      return
    res = common_listxattr(f, req, ino, list, size)
    if res > 0:
      fuse_reply_buf(req, list, res)
    else:
      reply_err(req, res)
    free(list)
  else:
    res = common_listxattr(f, req, ino, nil, 0)
    if res >= 0:
      fuse_reply_xattr(req, res)
    else:
      reply_err(req, res)

proc fuse_lib_removexattr*(req: fuse_req_t; ino: fuse_ino_t; name: cstring) =
  var f: ptr fuse = req_fuse_prepare(req)
  var path: cstring
  var err: cint
  err = get_path(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_removexattr(f.fs, path, name)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  reply_err(req, err)

proc locks_conflict*(node: ptr node; lock: ptr lock): ptr lock =
  var l: ptr lock
  l = node.locks
  while l:
    if l.owner != lock.owner and lock.start <= l.`end` and l.start <= lock.`end` and
        (l.`type` == F_WRLCK or lock.`type` == F_WRLCK):
      break
    l = l.next
  return l

proc delete_lock*(lockp: ptr ptr lock) =
  var l: ptr lock = lockp[]
  lockp[] = l.next
  free(l)

proc insert_lock*(pos: ptr ptr lock; lock: ptr lock) =
  lock.next = pos[]
  pos[] = lock

proc locks_insert*(node: ptr node; lock: ptr lock): cint =
  var lp: ptr ptr lock
  var newl1: ptr lock = nil
  var newl2: ptr lock = nil
  if lock.`type` != F_UNLCK or lock.start != 0 or lock.`end` != OFFSET_MAX:
    newl1 = malloc(sizeof(lock))
    newl2 = malloc(sizeof(lock))
    if not newl1 or not newl2:
      free(newl1)
      free(newl2)
      return -ENOLCK
  lp = addr(node.locks)
  while lp[]:
    var l: ptr lock = lp[]
    if l.owner != lock.owner:
      break skip
    if lock.`type` == l.`type`:
      if l.`end` < lock.start - 1:
        break skip
      if lock.`end` < l.start - 1:
        break
      if l.start <= lock.start and lock.`end` <= l.`end`:
        break `out`
      if l.start < lock.start:
        lock.start = l.start
      if lock.`end` < l.`end`:
        lock.`end` = l.`end`
      break delete
    else:
      if l.`end` < lock.start:
        break skip
      if lock.`end` < l.start:
        break
      if lock.start <= l.start and l.`end` <= lock.`end`:
        break delete
      if l.`end` <= lock.`end`:
        l.`end` = lock.start - 1
        break skip
      if lock.start <= l.start:
        l.start = lock.`end` + 1
        break
      newl2[] = l[]
      newl2.start = lock.`end` + 1
      l.`end` = lock.start - 1
      insert_lock(addr(l.next), newl2)
      newl2 = nil
    lp = addr(l.next)
    continue
    delete_lock(lp)
  if lock.`type` != F_UNLCK:
    newl1[] = lock[]
    insert_lock(lp, newl1)
    newl1 = nil
  free(newl1)
  free(newl2)
  return 0

proc flock_to_lock*(flock: ptr flock; lock: ptr lock) =
  memset(lock, 0, sizeof(lock))
  lock.`type` = flock.l_type
  lock.start = flock.l_start
  lock.`end` = if flock.l_len: flock.l_start + flock.l_len - 1 else: OFFSET_MAX
  lock.pid = flock.l_pid

proc lock_to_flock*(lock: ptr lock; flock: ptr flock) =
  flock.l_type = lock.`type`
  flock.l_start = lock.start
  flock.l_len = if (lock.`end` == OFFSET_MAX): 0 else: lock.`end` - lock.start + 1
  flock.l_pid = lock.pid

proc fuse_flush_common*(f: ptr fuse; req: fuse_req_t; ino: fuse_ino_t; path: cstring;
                       fi: ptr fuse_file_info): cint =
  var d: fuse_intr_data
  var lock: flock
  var l: lock
  var err: cint
  var errlock: cint
  fuse_prepare_interrupt(f, req, addr(d))
  memset(addr(lock), 0, sizeof((lock)))
  lock.l_type = F_UNLCK
  lock.l_whence = SEEK_SET
  err = fuse_fs_flush(f.fs, path, fi)
  errlock = fuse_fs_lock(f.fs, path, fi, F_SETLK, addr(lock))
  fuse_finish_interrupt(f, req, addr(d))
  if errlock != -ENOSYS:
    flock_to_lock(addr(lock), addr(l))
    l.owner = fi.lock_owner
    pthread_mutex_lock(addr(f.lock))
    locks_insert(get_node(f, ino), addr(l))
    pthread_mutex_unlock(addr(f.lock))
    ##  if op.lock() is defined FLUSH is needed regardless
    ## 		   of op.flush()
    if err == -ENOSYS:
      err = 0
  return err

proc fuse_lib_release*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var path: cstring
  var err: cint = 0
  get_path_nullok(f, ino, addr(path))
  if fi.flush:
    err = fuse_flush_common(f, req, ino, path, fi)
    if err == -ENOSYS:
      err = 0
  fuse_prepare_interrupt(f, req, addr(d))
  fuse_do_release(f, ino, path, fi)
  fuse_finish_interrupt(f, req, addr(d))
  free_path(f, ino, path)
  reply_err(req, err)

proc fuse_lib_flush*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var path: cstring
  var err: cint
  get_path_nullok(f, ino, addr(path))
  err = fuse_flush_common(f, req, ino, path, fi)
  free_path(f, ino, path)
  reply_err(req, err)

proc fuse_lock_common*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info;
                      lock: ptr flock; cmd: cint): cint =
  var f: ptr fuse = req_fuse_prepare(req)
  var path: cstring
  var err: cint
  err = get_path_nullok(f, ino, addr(path))
  if not err:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_lock(f.fs, path, fi, cmd, lock)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  return err

proc fuse_lib_getlk*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info;
                    lock: ptr flock) =
  var err: cint
  var l: lock
  var conflict: ptr lock
  var f: ptr fuse = req_fuse(req)
  flock_to_lock(lock, addr(l))
  l.owner = fi.lock_owner
  pthread_mutex_lock(addr(f.lock))
  conflict = locks_conflict(get_node(f, ino), addr(l))
  if conflict:
    lock_to_flock(conflict, lock)
  pthread_mutex_unlock(addr(f.lock))
  if not conflict:
    err = fuse_lock_common(req, ino, fi, lock, F_GETLK)
  else:
    err = 0
  if not err:
    fuse_reply_lock(req, lock)
  else:
    reply_err(req, err)

proc fuse_lib_setlk*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info;
                    lock: ptr flock; sleep: cint) =
  var err: cint = fuse_lock_common(req, ino, fi, lock, if sleep: F_SETLKW else: F_SETLK)
  if not err:
    var f: ptr fuse = req_fuse(req)
    var l: lock
    flock_to_lock(lock, addr(l))
    l.owner = fi.lock_owner
    pthread_mutex_lock(addr(f.lock))
    locks_insert(get_node(f, ino), addr(l))
    pthread_mutex_unlock(addr(f.lock))
  reply_err(req, err)

proc fuse_lib_flock*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info; op: cint) =
  var f: ptr fuse = req_fuse_prepare(req)
  var path: cstring
  var err: cint
  err = get_path_nullok(f, ino, addr(path))
  if err == 0:
    var d: fuse_intr_data
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_flock(f.fs, path, fi, op)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  reply_err(req, err)

proc fuse_lib_bmap*(req: fuse_req_t; ino: fuse_ino_t; blocksize: csize_t; idx: uint64_t) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var path: cstring
  var err: cint
  err = get_path(f, ino, addr(path))
  if not err:
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_bmap(f.fs, path, blocksize, addr(idx))
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  if not err:
    fuse_reply_bmap(req, idx)
  else:
    reply_err(req, err)

proc fuse_lib_ioctl*(req: fuse_req_t; ino: fuse_ino_t; cmd: cuint; arg: pointer;
                    llfi: ptr fuse_file_info; flags: cuint; in_buf: pointer;
                    in_bufsz: csize_t; out_bufsz: csize_t) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var fi: fuse_file_info
  var
    path: cstring
    out_buf: cstring = nil
  var err: cint
  err = -EPERM
  if flags and FUSE_IOCTL_UNRESTRICTED:
    break err
  if flags and FUSE_IOCTL_DIR:
    get_dirhandle(llfi, addr(fi))
  else:
    fi = llfi[]
  if out_bufsz:
    err = -ENOMEM
    out_buf = malloc(out_bufsz)
    if not out_buf:
      break err
  assert(not in_bufsz or not out_bufsz or in_bufsz == out_bufsz)
  if out_buf and in_bufsz:
    memcpy(out_buf, in_buf, in_bufsz)
  err = get_path_nullok(f, ino, addr(path))
  if err:
    break err
  fuse_prepare_interrupt(f, req, addr(d))
  err = fuse_fs_ioctl(f.fs, path, cmd, arg, addr(fi), flags,
                    if out_buf: out_buf else: cast[pointer](in_buf))
  fuse_finish_interrupt(f, req, addr(d))
  free_path(f, ino, path)
  if err < 0:
    break err
  fuse_reply_ioctl(req, err, out_buf, out_bufsz)
  break `out`
  reply_err(req, err)
  free(out_buf)

proc fuse_lib_poll*(req: fuse_req_t; ino: fuse_ino_t; fi: ptr fuse_file_info;
                   ph: ptr fuse_pollhandle) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var path: cstring
  var err: cint
  var revents: cuint = 0
  err = get_path_nullok(f, ino, addr(path))
  if not err:
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_poll(f.fs, path, fi, ph, addr(revents))
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  if not err:
    fuse_reply_poll(req, revents)
  else:
    reply_err(req, err)

proc fuse_lib_fallocate*(req: fuse_req_t; ino: fuse_ino_t; mode: cint; offset: off_t;
                        length: off_t; fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var path: cstring
  var err: cint
  err = get_path_nullok(f, ino, addr(path))
  if not err:
    fuse_prepare_interrupt(f, req, addr(d))
    err = fuse_fs_fallocate(f.fs, path, mode, offset, length, fi)
    fuse_finish_interrupt(f, req, addr(d))
    free_path(f, ino, path)
  reply_err(req, err)

proc fuse_lib_copy_file_range*(req: fuse_req_t; nodeid_in: fuse_ino_t; off_in: off_t;
                              fi_in: ptr fuse_file_info; nodeid_out: fuse_ino_t;
                              off_out: off_t; fi_out: ptr fuse_file_info;
                              len: csize_t; flags: cint) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var
    path_in: cstring
    path_out: cstring
  var err: cint
  var res: ssize_t
  err = get_path_nullok(f, nodeid_in, addr(path_in))
  if err:
    reply_err(req, err)
    return
  err = get_path_nullok(f, nodeid_out, addr(path_out))
  if err:
    free_path(f, nodeid_in, path_in)
    reply_err(req, err)
    return
  fuse_prepare_interrupt(f, req, addr(d))
  res = fuse_fs_copy_file_range(f.fs, path_in, fi_in, off_in, path_out, fi_out, off_out,
                              len, flags)
  fuse_finish_interrupt(f, req, addr(d))
  if res >= 0:
    fuse_reply_write(req, res)
  else:
    reply_err(req, res)
  free_path(f, nodeid_in, path_in)
  free_path(f, nodeid_out, path_out)

proc fuse_lib_lseek*(req: fuse_req_t; ino: fuse_ino_t; off: off_t; whence: cint;
                    fi: ptr fuse_file_info) =
  var f: ptr fuse = req_fuse_prepare(req)
  var d: fuse_intr_data
  var path: cstring
  var err: cint
  var res: off_t
  err = get_path(f, ino, addr(path))
  if err:
    reply_err(req, err)
    return
  fuse_prepare_interrupt(f, req, addr(d))
  res = fuse_fs_lseek(f.fs, path, off, whence, fi)
  fuse_finish_interrupt(f, req, addr(d))
  free_path(f, ino, path)
  if res >= 0:
    fuse_reply_lseek(req, res)
  else:
    reply_err(req, res)

proc clean_delay*(f: ptr fuse): cint =
  ##
  ##  This is calculating the delay between clean runs.  To
  ##  reduce the number of cleans we are doing them 10 times
  ##  within the remember window.
  ##
  var min_sleep: cint = 60
  var max_sleep: cint = 3600
  var sleep_time: cint = f.conf.remember div 10
  if sleep_time > max_sleep:
    return max_sleep
  if sleep_time < min_sleep:
    return min_sleep
  return sleep_time

proc fuse_clean_cache*(f: ptr fuse): cint =
  var lnode: ptr node_lru
  var
    curr: ptr list_head
    next: ptr list_head
  var node: ptr node
  var now: timespec
  pthread_mutex_lock(addr(f.lock))
  curr_time(addr(now))
  curr = f.lru_table.next
  while curr != addr(f.lru_table):
    var age: cdouble
    next = curr.next
    lnode = list_entry(curr, struct, node_lru, lru)
    node = addr(lnode.node)
    age = diff_timespec(addr(now), addr(lnode.forget_time))
    if age <= f.conf.remember:
      break
    assert(node.nlookup == 1)
    ##  Don't forget active directories
    if node.refctr > 1:
      curr = next
      continue
    node.nlookup = 0
    unhash_name(f, node)
    unref_node(f, node)
    curr = next
  pthread_mutex_unlock(addr(f.lock))
  return clean_delay(f)

var fuse_path_ops*: fuse_lowlevel_ops = [init: fuse_lib_init,
                                     destroy: fuse_lib_destroy,
                                     lookup: fuse_lib_lookup,
                                     forget: fuse_lib_forget,
                                     forget_multi: fuse_lib_forget_multi,
                                     getattr: fuse_lib_getattr,
                                     setattr: fuse_lib_setattr,
                                     access: fuse_lib_access,
                                     readlink: fuse_lib_readlink,
                                     mknod: fuse_lib_mknod, mkdir: fuse_lib_mkdir,
                                     unlink: fuse_lib_unlink,
                                     rmdir: fuse_lib_rmdir,
                                     symlink: fuse_lib_symlink,
                                     rename: fuse_lib_rename, link: fuse_lib_link,
                                     create: fuse_lib_create, open: fuse_lib_open,
                                     read: fuse_lib_read,
                                     write_buf: fuse_lib_write_buf,
                                     flush: fuse_lib_flush,
                                     release: fuse_lib_release,
                                     fsync: fuse_lib_fsync,
                                     opendir: fuse_lib_opendir,
                                     readdir: fuse_lib_readdir,
                                     readdirplus: fuse_lib_readdirplus,
                                     releasedir: fuse_lib_releasedir,
                                     fsyncdir: fuse_lib_fsyncdir,
                                     statfs: fuse_lib_statfs,
                                     setxattr: fuse_lib_setxattr,
                                     getxattr: fuse_lib_getxattr,
                                     listxattr: fuse_lib_listxattr,
                                     removexattr: fuse_lib_removexattr,
                                     getlk: fuse_lib_getlk, setlk: fuse_lib_setlk,
                                     flock: fuse_lib_flock, bmap: fuse_lib_bmap,
                                     ioctl: fuse_lib_ioctl, poll: fuse_lib_poll,
                                     fallocate: fuse_lib_fallocate,
                                     copy_file_range: fuse_lib_copy_file_range,
                                     lseek: fuse_lib_lseek]

proc fuse_notify_poll*(ph: ptr fuse_pollhandle): cint =
  return fuse_lowlevel_notify_poll(ph)

proc fuse_get_session*(f: ptr fuse): ptr fuse_session =
  return f.se

proc fuse_session_loop_remember*(f: ptr fuse): cint =
  var se: ptr fuse_session = f.se
  var res: cint = 0
  var now: timespec
  var next_clean: time_t
  var fds: pollfd = [fd: se.fd, events: POLLIN]
  var fbuf: fuse_buf = [mem: nil]
  curr_time(addr(now))
  next_clean = now.tv_sec
  while not fuse_session_exited(se):
    var timeout: cuint
    curr_time(addr(now))
    if now.tv_sec < next_clean:
      timeout = next_clean - now.tv_sec
    else:
      timeout = 0
    res = poll(addr(fds), 1, timeout * 1000)
    if res == -1:
      if errno == EINTR:
        continue
      else:
        break
    elif res > 0:
      res = fuse_session_receive_buf_int(se, addr(fbuf), nil)
      if res == -EINTR:
        continue
      if res <= 0:
        break
      fuse_session_process_buf_int(se, addr(fbuf), nil)
    else:
      timeout = fuse_clean_cache(f)
      curr_time(addr(now))
      next_clean = now.tv_sec + timeout
  free(fbuf.mem)
  fuse_session_reset(se)
  return if res < 0: -1 else: 0

proc fuse_loop*(f: ptr fuse): cint =
  if not f:
    return -1
  if lru_enabled(f):
    return fuse_session_loop_remember(f)
  return fuse_session_loop(f.se)

proc fuse_loop_mt_32*(f: ptr fuse; config: ptr fuse_loop_config): cint =
  if f == nil:
    return -1
  var res: cint = fuse_start_cleanup_thread(f)
  if res:
    return -1
  res = fuse_session_loop_mt_32(fuse_get_session(f), config)
  fuse_stop_cleanup_thread(f)
  return res

proc fuse_loop_mt_31*(f: ptr fuse; clone_fd: cint): cint
proc fuse_loop_mt_31*(f: ptr fuse; clone_fd: cint): cint =
  var config: fuse_loop_config
  config.clone_fd = clone_fd
  config.max_idle_threads = 10
  return fuse_loop_mt_32(f, addr(config))

proc fuse_exit*(f: ptr fuse) =
  fuse_session_exit(f.se)

proc fuse_get_context*(): ptr fuse_context =
  var c: ptr fuse_context_i = fuse_get_context_internal()
  if c:
    return addr(c.ctx)
  else:
    return nil

proc fuse_getgroups*(size: cint; list: ptr gid_t): cint =
  var c: ptr fuse_context_i = fuse_get_context_internal()
  if not c:
    return -EINVAL
  return fuse_req_getgroups(c.req, size, list)

proc fuse_interrupted*(): cint =
  var c: ptr fuse_context_i = fuse_get_context_internal()
  if c:
    return fuse_req_interrupted(c.req)
  else:
    return 0

proc fuse_invalidate_path*(f: ptr fuse; path: cstring): cint =
  var ino: fuse_ino_t
  var err: cint = lookup_path_in_cache(f, path, addr(ino))
  if err:
    return err
  return fuse_lowlevel_notify_inval_inode(f.se, ino, 0, 0)

var fuse_lib_opts*: UncheckedArray[fuse_opt] = [
    FUSE_OPT_KEY("debug", FUSE_OPT_KEY_KEEP),
    FUSE_OPT_KEY("-d", FUSE_OPT_KEY_KEEP),
    ["debug", offsetof(struct, fuse_config, debug), 1],
    ["-d", offsetof(struct, fuse_config, debug), 1],
    ["kernel_cache", offsetof(struct, fuse_config, kernel_cache), 1],
    ["auto_cache", offsetof(struct, fuse_config, auto_cache), 1],
    ["noauto_cache", offsetof(struct, fuse_config, auto_cache), 0],
    ["no_rofd_flush", offsetof(struct, fuse_config, no_rofd_flush), 1],
    ["umask=", offsetof(struct, fuse_config, set_mode), 1],
    ["umask=%o", offsetof(struct, fuse_config, umask), 0],
    ["uid=", offsetof(struct, fuse_config, set_uid), 1],
    ["uid=%d", offsetof(struct, fuse_config, uid), 0],
    ["gid=", offsetof(struct, fuse_config, set_gid), 1],
    ["gid=%d", offsetof(struct, fuse_config, gid), 0],
    ["entry_timeout=%lf", offsetof(struct, fuse_config, entry_timeout), 0],
    ["attr_timeout=%lf", offsetof(struct, fuse_config, attr_timeout), 0],
    ["ac_attr_timeout=%lf", offsetof(struct, fuse_config, ac_attr_timeout), 0], [
    "ac_attr_timeout=", offsetof(struct, fuse_config, ac_attr_timeout_set), 1], [
    "negative_timeout=%lf", offsetof(struct, fuse_config, negative_timeout), 0],
    ["noforget", offsetof(struct, fuse_config, remember), -1],
    ["remember=%u", offsetof(struct, fuse_config, remember), 0],
    ["modules=%s", offsetof(struct, fuse_config, modules), 0], FUSE_OPT_END]

proc fuse_lib_opt_proc*(data: pointer; arg: cstring; key: cint; outargs: ptr fuse_args): cint =
  cast[nil](arg)
  cast[nil](outargs)
  cast[nil](data)
  cast[nil](key)
  ##  Pass through unknown options
  return 1

var fuse_help_opts*: UncheckedArray[fuse_opt] = [
    ["modules=%s", offsetof(struct, fuse_config, modules), 1],
    FUSE_OPT_KEY("modules=%s", FUSE_OPT_KEY_KEEP), FUSE_OPT_END]

proc print_module_help*(name: cstring; fac: ptr fuse_module_factory_t) =
  var a: fuse_args = FUSE_ARGS_INIT(0, nil)
  if fuse_opt_add_arg(addr(a), "") == -1 or fuse_opt_add_arg(addr(a), "-h") == -1:
    return
  printf("\nOptions for %s module:\n", name)
  (fac[])(addr(a), nil)
  fuse_opt_free_args(addr(a))

proc fuse_lib_help*(args: ptr fuse_args) =
  ##  These are not all options, but only the ones that
  ## 	   may be of interest to an end-user
  printf("    -o kernel_cache        cache files in kernel\n    -o [no]auto_cache      enable caching based on modification times (off)\n    -o no_rofd_flush       disable flushing of read-only fd on close (off)\n    -o umask=M             set file permissions (octal)\n    -o uid=N               set file owner\n    -o gid=N               set file group\n    -o entry_timeout=T     cache timeout for names (1.0s)\n    -o negative_timeout=T  cache timeout for deleted names (0.0s)\n    -o attr_timeout=T      cache timeout for attributes (1.0s)\n    -o ac_attr_timeout=T   auto cache timeout for attributes (attr_timeout)\n    -o noforget            never forget cached inodes\n    -o remember=T          remember cached inodes for T seconds (0s)\n    -o modules=M1[:M2...]  names of modules to push onto filesystem stack\n")
  ##  Print low-level help
  fuse_lowlevel_help()
  ##  Print help for builtin modules
  print_module_help("subdir", addr(fuse_module_subdir_factory))
  when defined(HAVE_ICONV):
    print_module_help("iconv", addr(fuse_module_iconv_factory))
  ##  Parse command line options in case we need to
  ## 	   activate more modules
  var conf: fuse_config = [modules: nil]
  if fuse_opt_parse(args, addr(conf), fuse_help_opts, fuse_lib_opt_proc) == -1 or
      not conf.modules:
    return
  var module: cstring
  var next: cstring
  var m: ptr fuse_module
  ##  Iterate over all modules
  module = conf.modules
  while module:
    var p: cstring
    p = module
    while p[] and p[] != ':':
      ## ignored statement
      inc(p)
    next = if p[]: p + 1 else: nil
    p[] = '\x00'
    m = fuse_get_module(module)
    if m:
      print_module_help(module, addr(m.factory))
    module = next

proc fuse_init_intr_signal*(signum: cint; installed: ptr cint): cint =
  var old_sa: sigaction
  if sigaction(signum, nil, addr(old_sa)) == -1:
    perror("fuse: cannot get old signal handler")
    return -1
  if old_sa.sa_handler == SIG_DFL:
    var sa: sigaction
    memset(addr(sa), 0, sizeof(sigaction))
    sa.sa_handler = fuse_intr_sighandler
    sigemptyset(addr(sa.sa_mask))
    if sigaction(signum, addr(sa), nil) == -1:
      perror("fuse: cannot set interrupt signal handler")
      return -1
    installed[] = 1
  return 0

proc fuse_restore_intr_signal*(signum: cint) =
  var sa: sigaction
  memset(addr(sa), 0, sizeof(sigaction))
  sa.sa_handler = SIG_DFL
  sigaction(signum, addr(sa), nil)

proc fuse_push_module*(f: ptr fuse; module: cstring; args: ptr fuse_args): cint =
  var fs: array[2, ptr fuse_fs] = [f.fs, nil]
  var newfs: ptr fuse_fs
  var m: ptr fuse_module = fuse_get_module(module)
  if not m:
    return -1
  newfs = m.factory(args, fs)
  if not newfs:
    fuse_put_module(m)
    return -1
  newfs.m = m
  f.fs = newfs
  return 0

proc fuse_fs_new*(op: ptr fuse_operations; op_size: csize_t; user_data: pointer): ptr fuse_fs =
  var fs: ptr fuse_fs
  if sizeof(fuse_operations) < op_size:
    fuse_log(FUSE_LOG_ERR, "fuse: warning: library too old, some operations may not not work\n")
    op_size = sizeof(fuse_operations)
  fs = cast[ptr fuse_fs](calloc(1, sizeof(fuse_fs)))
  if not fs:
    fuse_log(FUSE_LOG_ERR, "fuse: failed to allocate fuse_fs object\n")
    return nil
  fs.user_data = user_data
  if op:
    memcpy(addr(fs.op), op, op_size)
  return fs

proc node_table_init*(t: ptr node_table): cint =
  t.size = NODE_TABLE_MIN_SIZE
  t.array = cast[ptr ptr node](calloc(1, sizeof(cast[ptr node](t.size[]))))
  if t.array == nil:
    fuse_log(FUSE_LOG_ERR, "fuse: memory allocation failed\n")
    return -1
  t.use = 0
  t.split = 0
  return 0

proc fuse_prune_nodes*(fuse: pointer): pointer =
  var f: ptr fuse = fuse
  var sleep_time: cint
  while 1:
    sleep_time = fuse_clean_cache(f)
    sleep(sleep_time)
  return nil

proc fuse_start_cleanup_thread*(f: ptr fuse): cint =
  if lru_enabled(f):
    return fuse_start_thread(addr(f.prune_thread), fuse_prune_nodes, f)
  return 0

proc fuse_stop_cleanup_thread*(f: ptr fuse) =
  if lru_enabled(f):
    pthread_mutex_lock(addr(f.lock))
    pthread_cancel(f.prune_thread)
    pthread_mutex_unlock(addr(f.lock))
    pthread_join(f.prune_thread, nil)

proc fuse_new_31*(args: ptr fuse_args; op: ptr fuse_operations; op_size: csize_t;
                 user_data: pointer): ptr fuse =
  var f: ptr fuse
  var root: ptr node
  var fs: ptr fuse_fs
  var llop: fuse_lowlevel_ops = fuse_path_ops
  f = cast[ptr fuse](calloc(1, sizeof(fuse)))
  if f == nil:
    fuse_log(FUSE_LOG_ERR, "fuse: failed to allocate fuse object\n")
    break `out`
  f.conf.entry_timeout = 1.0
  f.conf.attr_timeout = 1.0
  f.conf.negative_timeout = 0.0
  f.conf.intr_signal = FUSE_DEFAULT_INTR_SIGNAL
  ##  Parse options
  if fuse_opt_parse(args, addr(f.conf), fuse_lib_opts, fuse_lib_opt_proc) == -1:
    break out_free
  pthread_mutex_lock(addr(fuse_context_lock))
  var builtin_modules_registered: cint = 0
  ##  Have the builtin modules already been registered?
  if builtin_modules_registered == 0:
    ##  If not, register them.
    fuse_register_module("subdir", fuse_module_subdir_factory, nil)
    when defined(HAVE_ICONV):
      fuse_register_module("iconv", fuse_module_iconv_factory, nil)
    builtin_modules_registered = 1
  pthread_mutex_unlock(addr(fuse_context_lock))
  if fuse_create_context_key() == -1:
    break out_free
  fs = fuse_fs_new(op, op_size, user_data)
  if not fs:
    break out_delete_context_key
  f.fs = fs
  ##  Oh f**k, this is ugly!
  if not fs.op.lock:
    llop.getlk = nil
    llop.setlk = nil
  f.pagesize = getpagesize()
  init_list_head(addr(f.partial_slabs))
  init_list_head(addr(f.full_slabs))
  init_list_head(addr(f.lru_table))
  if f.conf.modules:
    var module: cstring
    var next: cstring
    module = f.conf.modules
    while module:
      var p: cstring
      p = module
      while p[] and p[] != ':':
        ## ignored statement
        inc(p)
      next = if p[]: p + 1 else: nil
      p[] = '\x00'
      if module[0] and fuse_push_module(f, module, args) == -1:
        break out_free_fs
      module = next
  if not f.conf.ac_attr_timeout_set:
    f.conf.ac_attr_timeout = f.conf.attr_timeout
  when defined(__FreeBSD__) or defined(__NetBSD__):
    ##
    ##  In FreeBSD, we always use these settings as inode numbers
    ##  are needed to make getcwd(3) work.
    ##
    f.conf.readdir_ino = 1
  f.se = fuse_session_new(args, addr(llop), sizeof((llop)), f)
  if f.se == nil:
    break out_free_fs
  if f.conf.debug:
    fuse_log(FUSE_LOG_DEBUG, "nullpath_ok: %i\n", f.conf.nullpath_ok)
  f.fs.debug = f.conf.debug
  f.ctr = 0
  f.generation = 0
  if node_table_init(addr(f.name_table)) == -1:
    break out_free_session
  if node_table_init(addr(f.id_table)) == -1:
    break out_free_name_table
  pthread_mutex_init(addr(f.lock), nil)
  root = alloc_node(f)
  if root == nil:
    fuse_log(FUSE_LOG_ERR, "fuse: memory allocation failed\n")
    break out_free_id_table
  if lru_enabled(f):
    var lnode: ptr node_lru = node_lru(root)
    init_list_head(addr(lnode.lru))
  strcpy(root.inline_name, "/")
  root.name = root.inline_name
  if f.conf.intr and
      fuse_init_intr_signal(f.conf.intr_signal, addr(f.intr_installed)) == -1:
    break out_free_root
  root.parent = nil
  root.nodeid = FUSE_ROOT_ID
  inc_nlookup(root)
  hash_id(f, root)
  return f
  free(root)
  free(f.id_table.array)
  free(f.name_table.array)
  fuse_session_destroy(f.se)
  if f.fs.m:
    fuse_put_module(f.fs.m)
  free(f.fs)
  free(f.conf.modules)
  fuse_delete_context_key()
  free(f)
  return nil

##  Emulates 3.0-style fuse_new(), which processes --help

proc fuse_new_30*(args: ptr fuse_args; op: ptr fuse_operations; op_size: csize_t;
                 private_data: pointer): ptr fuse
proc fuse_new_30*(args: ptr fuse_args; op: ptr fuse_operations; op_size: csize_t;
                 user_data: pointer): ptr fuse =
  var conf: fuse_config
  memset(addr(conf), 0, sizeof((conf)))
  var opts: UncheckedArray[fuse_opt] = [["-h",
                                     offsetof(struct, fuse_config, show_help), 1], [
      "--help", offsetof(struct, fuse_config, show_help), 1], FUSE_OPT_END]
  if fuse_opt_parse(args, addr(conf), opts, fuse_lib_opt_proc) == -1:
    return nil
  if conf.show_help:
    fuse_lib_help(args)
    return nil
  else:
    return fuse_new_31(args, op, op_size, user_data)

proc fuse_destroy*(f: ptr fuse) =
  var i: csize_t
  if f.conf.intr and f.intr_installed:
    fuse_restore_intr_signal(f.conf.intr_signal)
  if f.fs:
    fuse_create_context(f)
    i = 0
    while i < f.id_table.size:
      var node: ptr node
      node = f.id_table.array[i]
      while node != nil:
        if node.is_hidden:
          var path: cstring
          if try_get_path(f, node.nodeid, nil, addr(path), nil, false) == 0:
            fuse_fs_unlink(f.fs, path)
            free(path)
        node = node.id_next
      inc(i)
  i = 0
  while i < f.id_table.size:
    var node: ptr node
    var next: ptr node
    node = f.id_table.array[i]
    while node != nil:
      next = node.id_next
      free_node(f, node)
      dec(f.id_table.use)
      node = next
    inc(i)
  assert(list_empty(addr(f.partial_slabs)))
  assert(list_empty(addr(f.full_slabs)))
  while fuse_modules:
    fuse_put_module(fuse_modules)
  free(f.id_table.array)
  free(f.name_table.array)
  pthread_mutex_destroy(addr(f.lock))
  fuse_session_destroy(f.se)
  free(f.conf.modules)
  free(f)
  fuse_delete_context_key()

proc fuse_mount*(f: ptr fuse; mountpoint: cstring): cint =
  return fuse_session_mount(fuse_get_session(f), mountpoint)

proc fuse_unmount*(f: ptr fuse) =
  fuse_session_unmount(fuse_get_session(f))

proc fuse_version*(): cint =
  return FUSE_VERSION

proc fuse_pkgversion*(): cstring =
  return PACKAGE_VERSION
