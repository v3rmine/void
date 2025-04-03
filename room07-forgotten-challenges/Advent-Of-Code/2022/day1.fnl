(local u (require :utils))
(local stdin (u.io.lines))

(fn get-elfs []
  (let [max-by-elf {}]
    (var current-elf 1)
    (each [_ line (pairs stdin)]
      (if (< (length line) 1)
          (set current-elf (+ 1 current-elf))
          (tset max-by-elf current-elf
                (-> (or (?. max-by-elf current-elf) 0)
                    (+ (tonumber line))))))
    max-by-elf))

(local elfs (-> (get-elfs)
                (u.table.sort)
                (u.table.reverse)))

(local best-three (-> elfs (u.table.slice 1 3)))
(local sum-best-three (u.table.sum best-three))

(print (.. "Highest: " (. elfs 1)))
(print (.. "Sum best three: " sum-best-three))

