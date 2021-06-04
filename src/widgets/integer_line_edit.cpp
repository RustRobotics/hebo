// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "widgets/integer_line_edit.h"

#include <QDebug>
#include <QIntValidator>
#include <QKeyEvent>

#include "resources/images/images.h"

namespace hebo {
namespace {

constexpr const int kDefaultMin = 0;
constexpr const int kDefaultMax = 100;

}  // namespace

IntegerLineEdit::IntegerLineEdit(QWidget* parent)
  : QLineEdit(parent),
    validator_(new QIntValidator(kDefaultMin, kDefaultMax, this)) {
  this->initUi();
  this->initSignals();
}

void IntegerLineEdit::initUi() {
  this->setValidator(this->validator_);
//  this->plus_action_ = this->addAction(QIcon(kImageBlackPlus), QLineEdit::TrailingPosition);
//  this->minus_action_ = this->addAction(QIcon(kImageBlackMinus), QLineEdit::TrailingPosition);
}

void IntegerLineEdit::initSignals() {
  connect(this, &IntegerLineEdit::textChanged,
          this, &IntegerLineEdit::onTextChanged);
//  connect(this->plus_action_, &QAction::triggered,
//          this, &IntegerLineEdit::onPlusClicked);
//  connect(this->minus_action_, &QAction::triggered,
//          this, &IntegerLineEdit::onMinusClicked);
}

void IntegerLineEdit::setRange(int min, int max) {
  this->validator_->setRange(min, max);
}

int IntegerLineEdit::value() const {
  return this->text().toInt();
}

void IntegerLineEdit::setValue(int integer) {
  if (this->validateInteger(integer)) {
    this->setText(QString::number(integer));
  }
}

void IntegerLineEdit::onTextChanged(const QString& text) {
  const int integer = text.toInt();
  if (this->validateInteger(integer)) {
    emit this->valueChanged(integer);
  }
}

void IntegerLineEdit::onPlusClicked() {
  this->setValue(this->value() + 1);
}

void IntegerLineEdit::onMinusClicked() {
  this->setValue(this->value() - 1);
}

void IntegerLineEdit::keyPressEvent(QKeyEvent* event) {
  QLineEdit::keyPressEvent(event);
  switch (event->key()) {
    case Qt::Key_Up: {
      this->setValue(this->value() + 1);
      break;
    }
    case Qt::Key_Down: {
      this->setValue(this->value() - 1);
      break;
    }
    default: {
    }
  }
}

bool IntegerLineEdit::validateInteger(int integer) {
  return integer >= this->validator_->bottom() && integer <= this->validator_->top();
}

QSize IntegerLineEdit::sizeHint() const {
  const QSize default_size = QLineEdit::sizeHint();
  // TODO(Shaohua): Size hint based on range of value.
  return {108, default_size.height()};
}

}  // namespace hebo