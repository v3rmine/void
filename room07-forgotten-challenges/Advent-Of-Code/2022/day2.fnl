(local u (include :utils))

(fn get-self-play-value [line]
  (let [self (string.match line "%u? ?(%u)")]
    (match self
      "X" 1
      "Y" 2
      "Z" 3)))

(fn get-round-value [line]
  (match line
    (where (or "A Y" "B Z" "C X")) 6
    (where (or "A X" "B Y" "C Z")) 3
    (where (or "A Z" "B X" "C Y")) 0))

(fn get-results [stdin]
  (var results [])
  (each [idx line (pairs stdin)]
    (if (< 1 (length line))
        (let [play (get-self-play-value line)
              round (get-round-value line)]
          (table.insert
            results
            { :play play
              :round round
              :total (+ play round)}))))
  results)

(local stdin (u.io.lines))

(local plays (get-results stdin))
(local total-score (u.table.sum 
                     plays 
                     (fn [el] el.total)))
;; First star
(print (.. "First star total: " total-score))

(fn get-round-value-from-expected [line]
  (let [self (string.match line "%u? ?(%u)")]
    (match self
    "X" 0
    "Y" 3
    "Z" 6)))

(fn get-play-from-expected-result [line]
  (match line
    (where (or "A Y" "B X" "C Z")) "X"
    (where (or "A Z" "B Y" "C X")) "Y"
    (where (or "A X" "B Z" "C Y")) "Z"))

(fn get-results-from-expected [stdin]
  (var results [])
  (each [idx line (pairs stdin)]
    (if (< 1 (length line))
        (let [play (get-self-play-value 
                     (get-play-from-expected-result line))
              round (get-round-value-from-expected line)]
          (table.insert
            results
            { :play play
              :round round
              :total (+ play round)}))))
  results)

(local expected-plays (get-results-from-expected stdin))
(local expected-total-score (u.table.sum 
                     expected-plays 
                     (fn [el] el.total)))
;; Second star
(print (.. "Second star total: " expected-total-score))

