fn replace_with_84(s: &mut Box<i32>) {
    let was = std::mem::take(s);
    *s = was;
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variables() {
        let x = 42;
        let y = 43;
        let var1 = &x;
        let mut var2 = &x;
        var2 = &y;
        assert_eq!(y, *var2);
        assert_eq!(x, *var1);
    }

    #[test]
    fn test_ownership() {
        let x1 = 42;
        let y1 = Box::new(84);
        {
            let _z = (x1, y1);
        }
        assert_eq!(42, x1); 
    }

    #[test]
    fn test_replace_with_84() {
        let mut origin = Box::new(10);
        replace_with_84(&mut origin);
        assert_eq!(84, *origin);
    }
}