(local u (require :utils))
(local stdin (u.io.lines))

(fn get-boundaries [ass]
  (-> ass
      (u.table.string.split "[^-]+")
      (u.table.map (fn [str] (tonumber str)))
      (table.unpack)
      (values)))

(fn get-included-ass [lines]
  (var contained-ass [])
  (each [_ line (ipairs lines)]
    (let [(first-ass second-ass)
          (-> line
              (u.table.string.split "[^,]+")
              (table.unpack)
              (values))]
      (let [(first-ass-min first-ass-max) (get-boundaries first-ass)
            (second-ass-min second-ass-max) (get-boundaries second-ass)]
        (if (or
          (and (<= first-ass-min second-ass-min) (<= second-ass-max first-ass-max))
          (and (<= second-ass-min first-ass-min) (<= first-ass-max second-ass-max)))
            (table.insert contained-ass line)))))
  contained-ass)

(local included-ass (-> stdin
                        (u.table.string.non-empty)
                        (get-included-ass)))

;; First star
(print (.. "First star count is " (length included-ass)))

(fn get-overlapped-ass [lines]
  (var contained-ass [])
  (each [_ line (ipairs lines)]
    (let [(first-ass second-ass)
          (-> line
              (u.table.string.split "[^,]+")
              (table.unpack)
              (values))]
      (let [(first-ass-min first-ass-max) (get-boundaries first-ass)
            (second-ass-min second-ass-max) (get-boundaries second-ass)]
        (if (or
          (and (<= first-ass-max second-ass-max) (<= second-ass-min first-ass-max))
          (and (<= second-ass-max first-ass-max) (<= first-ass-min second-ass-max))
          (and (<= first-ass-min second-ass-min) (<= second-ass-min first-ass-max))
          (and (<= second-ass-min first-ass-min) (<= first-ass-min second-ass-max)))
            (table.insert contained-ass line)))))
  contained-ass)

(local overlapped-ass (-> stdin
                        (u.table.string.non-empty)
                        (get-overlapped-ass)))
;; Second star
(print (.. "Second star count is " (length overlapped-ass)))
