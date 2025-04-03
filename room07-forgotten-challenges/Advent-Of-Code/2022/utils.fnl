;; Generic utils
(fn dbg! [x]
  (let [fennel (require :fennel)]
    (print (fennel.view x))))

;; Io utils
(fn io-lines []
  (var new-table [])
  (each [line (io.lines)]
    (table.insert new-table line))
  new-table)

;; Table utils
(fn table-sum [iter ?mapper]
  (var sum 0)
  (each [_ value (pairs iter)]
    (set sum 
         (+ sum 
            (match ?mapper
              nil value
              _ (?mapper value)))))
  sum)

(fn table-slice [iter from to]
  (var new-table [])
  (for [idx from to]
    (table.insert new-table (?. iter idx)))
  new-table)

(fn table-map [iter mapper]
  (var new-table [])
  (each [_ value (pairs iter)]
    (table.insert new-table (mapper value)))
  new-table)

(fn table-filter [iter test]
  (var new-table [])
  (each [_ value (pairs iter)]
    (if (test value)
        (table.insert new-table value)))
  new-table)

(fn table-reverse [iter]
  (var new-table [])
  ;; REVIEW: Why here it start at 0
  (for [idx 0 (length iter)]
    (-> new-table
        (table.insert (. iter (- (length iter) idx)))))
  new-table)

;; Does no mutate
(fn table-sort [iter ...]
  (var new-table iter)
  (table.sort iter ...)
  new-table)

(fn table-split-at [iter at]
  (var first-half [])
  (var second-half [])
  (each [idx value (ipairs iter)]
    (if (<= idx at)
        (table.insert first-half value)
        (table.insert second-half value)))
  (values first-half second-half))

(fn table-find [iter val ?key]
  (var copy iter)
  (match (next copy ?key)
    (key val) key
    (key not-val) (table-find copy val key)))

(fn table-group-of [iter count]
  (var new-table [])
  (var group-idx 0)
  (each [idx value (ipairs iter)]
    (if (or (= 0 (% (- idx 1) count)) (= 1 idx))
        ((fn [] 
          (table.insert new-table [])
          (set group-idx (+ group-idx 1)))))
    (table.insert (. new-table group-idx) value))
  new-table)

(fn table-string-non-empty [iter]
  (-> iter (table-filter (fn [val] (< 1 (length val))))))

(fn table-string-join [iter ?sep]
  (var result "")
  (each [_ value (ipairs iter)]
    (set result 
         (.. result
             (match ?sep
               nil value
               sep (.. sep value)))))
  result)

(fn table-string-split [str pat]
  (icollect [v _ (string.gmatch str pat)] v))

{ :dbg! dbg! 
  :io {
    :lines io-lines }
  :table { 
    :sum table-sum
    :slice table-slice
    :filter table-filter
    :reverse table-reverse
    :sort table-sort
    :map table-map
    :split-at table-split-at
    :find table-find
    :group-of table-group-of
    :string {
      :split table-string-split
      :join table-string-join
      :non-empty table-string-non-empty }}}
