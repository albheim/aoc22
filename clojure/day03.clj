(def test-data "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw\n")

(defn priority [c]
  ; Take a character and return the priority of that item
  (let [l (first (clojure.string/lower-case c))]
    (if (= c l)
      (+ (- (int l) (int \a)) 1)
      (+ (- (int c) (int \A)) 27))))

(defn parse [input]
  ; Parse input line by line and replace items with their priorities
  (map (fn [line] 
         (map priority line)) 
       (clojure.string/split-lines input)))

(defn find-pocket-intersection [backpack]
  ; Split input in two and check intersection
  (let [halflen (/ (count backpack) 2)]
    (first 
      (clojure.set/intersection 
        (set (take halflen backpack)) 
        (set (drop halflen backpack))))))

(defn run-a [backpacks]
  ; Sum over all backpacks
  (reduce + (map find-pocket-intersection backpacks)))


(run-a (parse test-data))

(run-a (slurp "data/day03.txt"))

(defn find-intersection [backpacks]
  ; Find intersection over all backpacks in input
  (first (reduce clojure.set/intersection (map set backpacks))))

(defn run-b [backpacks]
  ; Take intersection of backpacks in groups of 3 and sum
  (reduce + 0 (map find-intersection (partition 3 backpacks))))

(run-b (parse test-data))

(run-b (slurp "data/day03.txt"))

