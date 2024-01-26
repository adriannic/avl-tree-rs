pub struct AVLTree {
    left: Option<Box<AVLTree>>,
    right: Option<Box<AVLTree>>,
    height: i64,
    value: i64,
}

impl AVLTree {
    pub fn new(value: i64) -> Self {
        Self {
            left: None,
            right: None,
            height: 0,
            value,
        }
    }

    pub fn add_value(&mut self, value: i64) {
        let side = if value <= self.value {
            &mut self.left
        } else {
            &mut self.right
        };

        if let Some(branch) = side {
            branch.add_value(value)
        } else {
            *side = Some(Box::new(AVLTree::new(value)))
        }

        self.update_height();
        self.balance(value);
    }

    pub fn get_values(&self) -> Vec<i64> {
        let mut values = vec![];
        self.get_values_aux(&mut values);
        values
    }

    pub fn height(&self) -> i64 {
        self.height
    }

    #[inline(always)]
    fn update_height_recursive(&mut self) {
        let get_height = |branch: &mut Box<AVLTree>| {
            branch.update_height();
            branch.height()
        };
        let left_height = self.left.as_mut().map_or(-1, get_height);
        let right_height = self.right.as_mut().map_or(-1, get_height);

        self.height = 1 + i64::max(left_height, right_height);
    }

    #[inline(always)]
    fn update_height(&mut self) {
        let get_height = |branch: &mut Box<AVLTree>| branch.height;
        let left_height = self.left.as_mut().map_or(-1, get_height);
        let right_height = self.right.as_mut().map_or(-1, get_height);

        self.height = 1 + i64::max(left_height, right_height);
    }

    fn balance(&mut self, value: i64) {
        let left_height = self.left.as_ref().map_or(-1, |value| value.height);
        let right_height = self.right.as_ref().map_or(-1, |value| value.height);
        let balance_factor = left_height - right_height;

        if balance_factor < -1 {
            if let Some(right) = self.right.as_mut() {
                if value <= right.value {
                    right.right_rotate();
                }
            }
            self.left_rotate();
        } else if balance_factor > 1 {
            if let Some(left) = self.left.as_mut() {
                if value > left.value {
                    left.left_rotate();
                }
            }
            self.right_rotate();
        }
        self.update_height_recursive();
    }

    fn left_rotate(&mut self) {
        let mut new_left = Box::new(AVLTree::new(self.value));
        new_left.left = self.left.take();
        self.left = Some(new_left);

        if let (Some(left), Some(right)) = (self.left.as_mut(), self.right.as_mut()) {
            self.value = right.value;
            left.right = right.left.take();
            self.right = right.right.take();
        }
    }

    fn right_rotate(&mut self) {
        let mut new_right = Box::new(AVLTree::new(self.value));
        new_right.right = self.right.take();
        self.right = Some(new_right);

        if let (Some(right), Some(left)) = (self.right.as_mut(), self.left.as_mut()) {
            self.value = left.value;
            right.left = left.right.take();
            self.left = left.left.take();
        }
    }

    fn get_values_aux(&self, values: &mut Vec<i64>) {
        if let Some(left) = self.left.as_ref() {
            left.get_values_aux(values);
        }
        values.push(self.value);
        if let Some(right) = self.right.as_ref() {
            right.get_values_aux(values);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let tree = AVLTree::new(5);
        assert_eq!(tree.get_values(), vec![5]);
    }

    #[test]
    fn insert_left() {
        let mut tree = AVLTree::new(1);
        tree.add_value(0);
        assert_eq!(tree.get_values(), vec![0, 1]);
        assert_eq!(tree.value, 1);
    }

    #[test]
    fn insert_right() {
        let mut tree = AVLTree::new(0);
        tree.add_value(1);
        assert_eq!(tree.get_values(), vec![0, 1]);
        assert_eq!(tree.value, 0);
    }

    #[test]
    fn left_rotate() {
        let mut tree = AVLTree::new(0);
        tree.add_value(1);
        tree.add_value(2);
        assert_eq!(tree.get_values(), vec![0, 1, 2]);
        assert_eq!(tree.height, 1);
        assert_eq!(tree.value, 1);
    }

    #[test]
    fn right_rotate() {
        let mut tree = AVLTree::new(2);
        tree.add_value(1);
        tree.add_value(0);
        assert_eq!(tree.get_values(), vec![0, 1, 2]);
        assert_eq!(tree.height, 1);
        assert_eq!(tree.value, 1);
    }
}
