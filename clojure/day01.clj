(def day "day01")
(def test-data (slurp (reduce str ["data/" day ".test"])))
(def real-data (slurp (reduce str ["data/" day ".data"])))

(defn split-list [pred items]
  ; Split list on items where pred is true, remove those items
  (apply conj (reduce (fn [[acc curr] x]
                        (if (pred x)
                          [(conj acc curr) []]
                          [acc (conj curr x)]))
                      [[] []] items)))

(defn parse [input]
  ; Split into lists of calories for each elf
  (map (fn [x] (map #(Integer/parseInt %) x)) 
       (split-list empty? (clojure.string/split-lines input))))

(defn run-a [elfs]
  ; Max over each elfs total calories
  (apply max (map #(reduce + %) elfs)))

(run-a (parse test-data))

(println "Part a:" (run-a (parse real-data)))

(defn run-b [elfs]
  ; Sum of 3 largest total calories
  (reduce + (take 3 (reverse (sort (map #(reduce + %) elfs))))))

(run-b (parse test-data))

(println "Part b:" (run-b (parse real-data)))

