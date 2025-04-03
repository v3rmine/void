(local u (require :utils))
(local stdin (u.io.lines))

(fn get-value [char]
  (match (string.match char "%l")
    ml (- (string.byte ml) (string.byte "a") -1)
    nil (match (string.match char "%u")
          mu (- (string.byte mu) (string.byte "A") -27)
          nil nil)))
  
(fn find-priority [raw-rucksack]
  (local (rucksack-first rucksack-second) 
         (let [rucksack (u.table.string.split raw-rucksack ".")]
           (-> rucksack
               (u.table.split-at (/ (length rucksack) 2)))))
  (var (idx item) (next rucksack-first))
  (var found nil)
  (while (and (not= nil item) (= nil found))
    (let [search (u.table.find rucksack-second item)]
      (if (not= nil search)
        (set found (. rucksack-second search))
        (set (idx item) (next rucksack-first idx)))))
  (get-value found))

(local priorities (-> stdin 
                      (u.table.string.non-empty)
                      (u.table.map find-priority)))
(local priorities-sum (u.table.sum priorities))

;; First star
(print (.. "First star sum is: " priorities-sum))

(local groups (-> stdin 
                  (u.table.string.non-empty)
                  (u.table.group-of 3)))

(fn find-badge [group]
  (local rucksacks (u.table.map group
                                (fn [rucksack] 
                                  (u.table.string.split rucksack "."))))
  (var (first-idx first-rucksack) (next rucksacks))
  (var (item-idx item) (next first-rucksack))
  (var (rucksack-idx rucksack) (next rucksacks first-idx))
  (var found nil)

  (while (and (not= nil item) (= nil found))
    (var found-in-rucksack nil)
    (while (and (not= nil rucksack) (= nil found-in-rucksack))
      (let [search (u.table.find rucksack item)]
        (if (not= nil search)
            (set found-in-rucksack search)
            ((fn []
               ;; Check next item
               (set (item-idx item) (next first-rucksack item-idx))
               ;; Reset the rucksack
               (set (rucksack-idx rucksack) (next rucksacks)))))))

    (if (not= nil found-in-rucksack)
        ((fn [] 
           (if (= (length rucksacks) rucksack-idx)
               (set found (. rucksack found-in-rucksack)))
           (set (rucksack-idx rucksack) (next rucksacks rucksack-idx))))))
  
  (get-value found))

(local badges-sum (-> groups
                      (u.table.map (fn [group] (find-badge group)))
                      (u.table.sum)))

;; Second star
(print (.. "Second star sum is: " badges-sum))

